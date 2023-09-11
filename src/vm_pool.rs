//! The virtual_machine module allows to interact with OpenNebula virtual_machines

use std::fmt::Display;

use xml_doc::{Document, Element};

use crate::common::parameters::UpdateType;
use crate::common::permissions::{Permissions, PermissionsBits};
use crate::common::resource::{Resource, ResourceGetter, ResourceGetterMut};
use crate::common::resource_getters::{CommonGetters, GetGroup, GetOwner, GetPermissions};
use crate::common::resource_pool::{build_pool, ResourcePool};
use crate::common::template_getters::TemplateCommonGetters;
use crate::common::template_mut::TemplateMut;
use crate::common::{Errors, Template};
use crate::controller::{Controller, RPCCaller};
use crate::{rpc_chmod_method, vm};

#[derive(Debug)]
pub struct VirtualMachinesController<'a, C: RPCCaller> {
    pub controller: &'a Controller<C>,
}

pub struct VirtualMachinePool {
    resource: ResourcePool,
}

impl ResourceGetter for VirtualMachinePool {
    fn get_internal(&self) -> (&Document, &Element) {
        (&self.resource.document, &self.resource.root)
    }
}

impl GetGroup for VirtualMachinePool {}
impl GetOwner for VirtualMachinePool {}

impl Display for VirtualMachinePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.resource.document.write_str().unwrap())
    }
}

impl vm::VMShared for VirtualMachinePool {}

impl<'a, C: RPCCaller> VirtualMachinesController<'a, C> {
    pub fn allocate<T: TemplateCommonGetters<'a> + Display>(
        &self,
        template: T,
        pending: bool,
    ) -> Result<i32, Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.allocate",
            vec![template.to_string().into(), pending.into()],
        )?;

        let id = self.controller.parse_id_resp(resp_txt)?;

        Ok(id)
    }

    pub fn info(&self) -> Result<Vec<VirtualMachinePool>, Errors> {
        let resp_txt = self.controller.client.call(
            "one.vmpool.info",
            vec![(-1).into(), (-1).into(), (-1).into(), (-1).into()],
        )?;

        let body = self.controller.parse_body_resp(resp_txt)?;

        let mut vms = Vec::new();

        match build_pool(body.as_str(), "VM") {
            Ok(elements) => {
                for vm in elements {
                    vms.push(VirtualMachinePool {
                        resource: ResourcePool {
                            document: vm.document,
                            root: vm.root,
                        },
                    })
                }
                Ok(vms)
            }
            Err(e) => Err(Errors::Roca(format!(
                "Failed to parse the resource pool: {}",
                e
            ))),
        }
    }

    /*

        Missing methods:

        vmpool.infoextended
        vmpool.infoset
        vmpool.monitoring
        vmpool.accounting
        vmpool.showback
        vmpool.calculateshowback
    */
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::prelude::*;
    use crate::vm::{Action, VirtualMachineController};

    fn create_vm(controller: &Controller<ClientXMLRPC>, name: &str) -> i32 {
        let mut tpl = TemplateBuilder::new();
        tpl.put_str("NAME", name);
        tpl.put_str("CPU", "1");
        tpl.put_str("MEMORY", "32");

        // add custom pairs with same key, be careful, keys will be renamed uppercase
        tpl.put_str("custom", "test");
        tpl.put_str("CUSTOM", "test2");

        let allocate_response = controller.virtual_machines().allocate(tpl, false);

        println!("{:?}", allocate_response);
        assert!(allocate_response.is_ok());
        let vm_id = allocate_response.unwrap();
        assert!(vm_id > 0);

        vm_id
    }

    fn destroy_vm(vm_controller: VirtualMachineController<ClientXMLRPC>) {
        // Terminate the virtual_machine
        let terminate_response = vm_controller.action(Action::TerminateHard);
        println!("{:?}", terminate_response);

        assert!(terminate_response.is_ok());
    }

    #[test]
    fn virtual_machine_pool() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:pDi4mFBHue"),
            String::from("http://192.168.33.10:2633/RPC2"),
        );

        // Create the virtual_machine
        let controller = Controller::new(client);

        let vm_id = create_vm(&controller, "roca-test-vm-pool");
        let vm_controller = controller.virtual_machine(vm_id);

        let pool_infos = controller.virtual_machines().info();
        assert!(pool_infos.is_ok());

        let vms = pool_infos.unwrap();

        for vm in vms {
            // look for our VM in the pool
            assert!(vm.name().is_ok());

            if vm.name().unwrap() != "roca-test-vm-pool" {
                continue;
            }

            assert!(vm.id().is_ok());
            assert!(vm.id().unwrap() > 0);

            assert!(vm.gid().is_ok());
            assert_eq!(vm.gid().unwrap(), 0);

            assert!(vm.groupname().is_ok());
            assert_eq!(vm.groupname().unwrap(), "oneadmin".to_owned());

            println!("{}", vm);

            // retrieve first pair with "custom" key
            let memory = vm.template().get_i64("MEMORY");
            assert!(memory.is_ok());
            assert_eq!(memory.unwrap(), 32);
        }

        destroy_vm(vm_controller)
    }
}

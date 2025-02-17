//! The virtual_machine module allows to interact with OpenNebula virtual_machines

use std::fmt::Display;

use crate::common::parameters::UpdateType;
use crate::common::permissions::{Permissions, PermissionsBits};
use crate::common::resource_getters::{GetGroup, GetOwner, GetPermissions};
use crate::common::Errors;
use crate::controller::{Controller, RPCCaller};
use crate::rpc_chmod_method;

use crate::common::xml::shared_getters::BaseGetters;
use crate::common::xml::template::Template;
use crate::{common::xml::resource::Resource, define_resource};

#[derive(Debug)]
pub struct VirtualMachineController<'a, C: RPCCaller> {
    pub controller: &'a Controller<C>,
    pub id: i32,
}

#[derive(Debug)]
pub struct VMDiskController<'a, C: RPCCaller> {
    pub vm_controller: &'a VirtualMachineController<'a, C>,
    pub id: i32,
}

#[derive(Debug)]
pub struct VMNICController<'a, C: RPCCaller> {
    pub vm_controller: &'a VirtualMachineController<'a, C>,
    pub id: i32,
}

define_resource!(VirtualMachine);

impl GetGroup for VirtualMachine {}
impl GetOwner for VirtualMachine {}
impl GetPermissions for VirtualMachine {}

// Shared behavior between VirtualMachine and VirtualMachinePool
pub trait VMShared: XMLDocGetters {
    /// Allow to retrieve the user template section of the VM
    fn user_template(&self) -> Template {
        let (document, element) = self.get_internal();

        let template = element.find(document, "USER_TEMPLATE").unwrap();

        Template::from_resource(document, template)
    }

    // TODO: return state string
    fn state(&self) -> &'static str {
        todo!()
    }
}

impl VMShared for VirtualMachine {}

pub enum Action {
    Terminate,
    TerminateHard,
    Undeploy,
    UndeployHard,
    PowerOff,
    PowerOffHard,
    Reboot,
    RebootHard,
    Hold,
    Release,
    Stop,
    Suspend,
    Resume,
    Resched,
    UnResched,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Terminate => f.write_str("terminate"),
            Action::TerminateHard => f.write_str("terminate-hard"),
            Action::Undeploy => f.write_str("undeploy"),
            Action::UndeployHard => f.write_str("undeploy-hard"),
            Action::PowerOff => f.write_str("poweroff"),
            Action::PowerOffHard => f.write_str("poweroff-hard"),
            Action::Reboot => f.write_str("reboot"),
            Action::RebootHard => f.write_str("reboot-hard"),
            Action::Hold => f.write_str("hold"),
            Action::Release => f.write_str("release"),
            Action::Stop => f.write_str("stop"),
            Action::Suspend => f.write_str("suspend"),
            Action::Resume => f.write_str("resume"),
            Action::Resched => f.write_str("resched"),
            Action::UnResched => f.write_str("unresched"),
        }
    }
}

impl<'a, C: RPCCaller> VirtualMachineController<'a, C> {
    // TODO: add a method per action ?
    /// Action is the generic method to run any action on the VM
    pub fn action(&self, action: Action) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.action",
            vec![action.to_string().into(), self.id.into()],
        )?;

        self.controller.parse_id_resp(resp_txt)?;

        Ok(())
    }

    /// Deploy in the selected hostID and/or dsID. Enforce to return error in case of
    /// overcommitment. Enforce is automatically enabled for non-oneadmin users.
    /// Set dsID to -1 to let OpenNebula choose the datastore.
    pub fn deploy(&self, host_id: i32, enforce: bool, ds_id: i32) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.deploy",
            vec![self.id.into(), host_id.into(), enforce.into(), ds_id.into()],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Migrate a VM to a target host and/or to another ds
    pub fn migrate(
        &self,
        host_id: i32,
        live: bool,
        enforce: bool,
        ds_id: i32,
        migration_type: i32,
    ) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.migrate",
            vec![
                self.id.into(),
                host_id.into(),
                live.into(),
                enforce.into(),
                ds_id.into(),
                migration_type.into(),
            ],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Attach a new disk
    pub fn disk_attach(&self, disk_template: &str) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.attach", vec![self.id.into(), disk_template.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /// Return a disk controller allowing to manage the disk
    pub fn disk(&'a self, disk_id: i32) -> VMDiskController<'a, C> {
        VMDiskController {
            vm_controller: self,
            id: disk_id,
        }
    }

    /// Attach a new NIC
    pub fn nic_attach(&self, nic_template: &str) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.attachnic",
            vec![self.id.into(), nic_template.into()],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Return a nic controller allowing to manage the disk
    pub fn nic(&'a self, nic_id: i32) -> VMNICController<'a, C> {
        VMNICController {
            vm_controller: self,
            id: nic_id,
        }
    }

    pub fn secgroup_attach(&self, secgroup_id: i32) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.attachsg", vec![self.id.into(), secgroup_id.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    pub fn secgroup_detach(&self) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.detachsg", vec![self.id.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    rpc_chmod_method!("one.vm.chmod");

    /// Changes the owner/group of a VM. If uid or gid is -1 it will not change
    pub fn chown(&self, uid: i32, gid: i32) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.chown", vec![self.id.into(), uid.into(), gid.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /// Changes the name of a VM
    pub fn rename(&self, new_name: &str) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.rename", vec![self.id.into(), new_name.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /// Changes the capacity of the virtual machine
    pub fn resize(&self, template: &str, enforce: bool) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.resize",
            vec![self.id.into(), template.into(), enforce.into()],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Adds VM content
    /// * vm_tpl: The new vm contents. Syntax can be the usual attribute=value or XML.
    /// * policy: Update type: 0: Replace the whole template. 1: Merge new template with the existing one.
    pub fn update<T: BaseGetters + Display>(
        &self,
        vm_tpl: T,
        policy: UpdateType,
    ) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.update",
            vec![
                self.id.into(),
                vm_tpl.to_string().into(),
                policy.value().into(),
            ],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Udates (appends) a set of supported configuration attributes in
    /// the VM template
    pub fn update_conf<T: BaseGetters + Display>(&self, vm_tpl: T) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.updateconf",
            vec![self.id.into(), vm_tpl.to_string().into()],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Info connects to OpenNebula and fetches the information of the VM
    pub fn info(&self) -> Result<VirtualMachine, Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.info", vec![self.id.into()])?;

        let body = self.controller.parse_body_resp(resp_txt)?;
        match Resource::try_from(body.as_str()) {
            Ok(resource) => Ok(VirtualMachine { resource }),
            Err(e) => Err(Errors::Roca(format!("Failed to parse the resource: {}", e))),
        }
    }

    // TODO: add an enum for op
    /// Recovers a stuck VM that is waiting for a driver operation
    pub fn recover(&self, op: i32) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.recover", vec![self.id.into(), op.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    // TODO: add an enum for lock level
    /// Locks the vm following lock level.
    pub fn lock(&self, level: i32) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.lock", vec![self.id.into(), level.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /// Unlock unlocks the vm.
    pub fn unlock(&self) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.unlock", vec![self.id.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /// adds a new scheduled action to the VM
    pub fn sched_add<T: BaseGetters + Display>(&self, action_tpl: T) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.schedadd",
            vec![self.id.into(), action_tpl.to_string().into()],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Updates the scheduled action specified by the action ID attribute
    pub fn sched_update<T: BaseGetters + Display + 'a>(&self, action_tpl: T) -> Result<(), Errors> {
        let action_id = action_tpl.get_i64("ID")?;

        let resp_txt = self.controller.client.call(
            "one.vm.schedupdate",
            vec![
                self.id.into(),
                action_id.into(),
                action_tpl.to_string().into(),
            ],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Deletes the actionId action
    pub fn sched_delete(&self, action_id: i32) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.scheddelete", vec![self.id.into(), action_id.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /// Backup virtual machine
    pub fn backup(&self, ds_id: i32, reset: bool) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.vm.backup",
            vec![self.id.into(), ds_id.into(), reset.into()],
        )?;

        self.controller.parse_resp(resp_txt)
    }

    /// Cancel ongoing backup operation
    pub fn backup_cancel(&self) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.vm.backupcancel", vec![self.id.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    /*
        Missing methods:

        monitoring
    */
}

impl<'a, C: RPCCaller> VMNICController<'a, C> {
    /// detaches a network interface from the virtual machine
    pub fn detach(&self) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.detachnic",
            vec![self.vm_controller.id.into(), self.id.into()],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Updates (appends) a NIC attributes
    /// * nic_tpl: The new nic contents. Syntax can be the usual attribute=value or XML.
    /// * policy: Update type: 0: Replace the whole NIC. 1: Merge new NIC with the existing one.
    pub fn update<T: BaseGetters + Display>(
        &self,
        nic_tpl: T,
        policy: UpdateType,
    ) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.updatenic",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                nic_tpl.to_string().into(),
                policy.value().into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }
}

impl<'a, C: RPCCaller> VMDiskController<'a, C> {
    /// Detach a disk from a virtual machine
    pub fn detach(&self) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.detach",
            vec![self.vm_controller.id.into(), self.id.into()],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Resize a disk of a virtual machine
    pub fn resize(&self, new_size: i64) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.resize",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                new_size.into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Exports a disk to an image and returns the image ID.
    /// If imageType is empty the default one will be used.
    /// If snapID is -1 the current image state will be exported
    pub fn saveas(&self, image_name: i32, image_type: &str, snap_id: i32) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.disksaveas",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                image_name.into(),
                image_type.into(),
                snap_id.into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Creates a new virtual machine snapshot. name can be empty
    pub fn snapshot_create(&self, disk_id: i32, desc: &str) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.disksnapshotcreate",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                disk_id.into(),
                desc.into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Deletes a virtual machine snapshot
    pub fn snapshot_delete(&self, disk_id: i32, snap_id: i32) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.disksnapshotdelete",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                disk_id.into(),
                snap_id.into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Revert disk state to a previously taken snapshot
    pub fn snapshot_revert(&self, disk_id: i32, snap_id: i32) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.disksnapshotrevert",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                disk_id.into(),
                snap_id.into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }

    /// Renames a snapshot
    pub fn snapshot_rename(
        &self,
        disk_id: i32,
        snap_id: i32,
        new_name: &str,
    ) -> Result<(), Errors> {
        let resp_txt = self.vm_controller.controller.client.call(
            "one.vm.disksnapshotrename",
            vec![
                self.vm_controller.id.into(),
                self.id.into(),
                disk_id.into(),
                snap_id.into(),
                new_name.into(),
            ],
        )?;

        self.vm_controller.controller.parse_resp(resp_txt)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        common::permissions::flags::{GRP_A, OTH_UMA, USR_UMA},
        prelude::*,
    };

    fn create_vm(controller: &Controller<ClientXMLRPC>, name: &str) -> i32 {
        let mut tpl = template::Builder::new();
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
    fn virtual_machine_complex() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:pDi4mFBHue"),
            String::from("http://192.168.33.10:2633/RPC2"),
        );

        // Create the virtual_machine
        let controller = Controller::new(client);

        let vm_id = create_vm(&controller, "roca-test-vm");

        let vm_controller = controller.virtual_machine(vm_id);

        // let's modify permissions
        let chmod_response = vm_controller.chmod(Permissions(USR_UMA | GRP_A | OTH_UMA));
        println!("{:?}", chmod_response);
        assert!(chmod_response.is_ok());

        // check elements values
        let infos = vm_controller.info();
        match infos {
            Ok(infos) => {
                assert!(infos.id().is_ok());
                assert!(infos.id().unwrap() > 0);

                assert!(infos.name().is_ok());
                assert_eq!(infos.name().unwrap(), "roca-test-vm");

                assert!(infos.gid().is_ok());
                assert_eq!(infos.gid().unwrap(), 0);

                assert!(infos.groupname().is_ok());
                assert_eq!(infos.groupname().unwrap(), "oneadmin");

                let perms = infos.permissions();
                assert!(perms.is_ok());
                assert_eq!(perms.unwrap().to_string(), "uma--auma");

                // retrieve first pair with "custom" key
                let user_tpl = infos.user_template();
                let custom_key = user_tpl.get("CUSTOM");
                println!("custom key: {:?}", custom_key);
                assert!(custom_key.is_ok());
                assert_eq!(custom_key.unwrap(), "test");

                // modify the template content
                let infos = infos;
                let mut builder: template::Builder = infos.user_template().into();
                assert!(builder.rm("CUSTOM").is_ok());

                //let tpl = infos.user_template();
                let custom_key = builder.get("CUSTOM");
                assert!(custom_key.is_err());
            }
            Err(e) => panic!("Error on virtual_machine info: {}", e),
        }

        destroy_vm(vm_controller);
    }
}

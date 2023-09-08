//! The user module allows to interact with OpenNebula users

use std::fmt::Display;

use crate::common::parameters::UpdateType;
use crate::common::resource::{Resource, ResourceGetter, ResourceGetterMut};
use crate::common::resource_getters::{CommonGetters, GetGroup};
use crate::common::template_getters::TemplateCommonGetters;
use crate::common::Errors;
use crate::controller::{Controller, RPCCaller};

// TODO: remove this /
use crate::rpc_noparam_method;

#[derive(Debug)]
pub struct UserController<'a, C: RPCCaller> {
    pub controller: &'a Controller<C>,
    pub id: i32,
}

#[derive(Debug)]
pub struct UsersController<'a, C: RPCCaller> {
    pub controller: &'a Controller<C>,
}

pub struct User {
    resource: Resource,
}

// read only
impl ResourceGetter for User {
    fn get_internal(&self) -> (&xml_doc::Document, &xml_doc::Element) {
        (&self.resource.document, &self.resource.root)
    }
}

// read-write
// read-write
impl ResourceGetterMut for User {
    fn get_internal_mut(&mut self) -> (&mut xml_doc::Document, &mut xml_doc::Element) {
        (&mut self.resource.document, &mut self.resource.root)
    }
}

impl GetGroup for User {}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.resource.document.write_str().unwrap())
    }
}

//https://docs.opennebula.io/6.4/installation_and_configuration/authentication/overview.html
// or look at the opennebula terraform provider code

//enum Authentication {
//    Sunstone,
//    OpenNebula,
//    x509,
//}

// TODOs:
// - enum form auth_drv options
// - more helpers without some options (i.e groups, auth_drv)
// - add last parameter: array of groups. Currently the client only support Array(Vec<Value>)
impl<'a, C: RPCCaller> UsersController<'a, C> {
    pub fn allocate(&self, name: &str, passwd: &str, auth_drv: &str) -> Result<i32, Errors> {
        let resp_txt = self.controller.client.call(
            "one.user.allocate",
            vec![name.into(), passwd.into(), auth_drv.into()],
        )?;

        let id = self.controller.parse_id_resp(resp_txt)?;

        Ok(id)
    }
}

impl<'a, C: RPCCaller> UserController<'a, C> {
    //pub fn delete(&self) -> Result<(), Errors> {
    //    let (success, err) = self
    //        .controller
    //        .client
    //        .call("one.user.delete", vec![self.id.into()])?;
    //
    //    self.controller.parse_resp(resp_txt)
    //}
    rpc_noparam_method!(delete, "one.user.delete");

    pub fn info(&self) -> Result<User, Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.user.info", vec![self.id.into()])?;

        let body = self.controller.parse_body_resp(resp_txt)?;
        match Resource::try_from(body.as_str()) {
            Ok(resource) => Ok(User { resource }),
            Err(e) => Err(Errors::Roca(format!("Failed to parse the resource: {}", e))),
        }
    }

    /// Updates adds user content
    /// * tpl: The new user contents. Syntax can be the usual attribute=value or XML.
    /// * policy: Update type: 0: Replace the whole template. 1: Merge new template with the existing one.
    pub fn update<T: TemplateCommonGetters<'a> + Display>(
        &self,
        tpl: T,
        policy: UpdateType,
    ) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.user.update",
            vec![
                self.id.into(),
                tpl.to_string().into(),
                policy.value().into(),
            ],
        )?;

        self.controller.parse_id_resp(resp_txt)?;

        Ok(())
    }

    pub fn passwd(&self, new_passd: i32) -> Result<(), Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.user.passwd", vec![self.id.into(), new_passd.into()])?;

        self.controller.parse_resp(resp_txt)
    }

    pub fn login(&self, name: &str, token: &str, period: i32, gid: i32) -> Result<String, Errors> {
        let resp_txt = self.controller.client.call(
            "one.user.login",
            vec![name.into(), token.into(), period.into(), gid.into()],
        )?;

        self.controller.parse_body_resp(resp_txt)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::client::ClientXMLRPC;

    #[test]
    fn user_info() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2"),
        );
        let controller = Controller::new(client);
        let user_controller = controller.user(0);

        match user_controller.info() {
            Ok(infos) => {
                assert!(infos.id().is_ok());
                assert_eq!(infos.id().unwrap(), 0);

                assert!(infos.name().is_ok());
                assert_eq!(infos.name().unwrap(), "oneadmin".to_owned());

                assert!(infos.gid().is_ok());
                assert_eq!(infos.gid().unwrap(), 0);

                assert!(infos.groupname().is_ok());
                assert_eq!(infos.groupname().unwrap(), "oneadmin".to_owned());

                assert!(infos.get_str("AUTH_DRIVER").is_ok());
                assert_eq!(infos.get_str("AUTH_DRIVER").unwrap(), "core".to_owned());
            }
            Err(e) => panic!("Error on user info: {}", e),
        }
    }

    #[test]
    fn user_allocate_delete() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2"),
        );

        // Create the user
        let controller = Controller::new(client);

        let allocate_response = controller.users().allocate("test-alloc", "test-alloc", "");

        assert!(allocate_response.is_ok());
        let user_id = allocate_response.unwrap();
        assert!(user_id > 0);

        let ucontroller = controller.user(user_id);

        // Delete the user
        let delete_response = ucontroller.delete();
        assert!(delete_response.is_ok());
    }

    #[test]
    fn user_login() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2"),
        );
        let controller = Controller::new(client);

        // Create the user
        let name = "test-login4";
        let allocate_response = controller.users().allocate(name, "password", "core");
        assert!(allocate_response.is_ok());
        let user_id = allocate_response.unwrap();
        assert!(user_id > 0);

        let ucontroller = controller.user(user_id);

        // Test loging
        let login_response = ucontroller.login(name, "password", 60, 0);
        assert!(login_response.is_ok());

        // Delete the user
        let delete_response = ucontroller.delete();
        assert!(delete_response.is_ok());
    }
}

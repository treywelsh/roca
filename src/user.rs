//! The user module allows to interact with OpenNebula users

use crate::common::getters::ResourceInternal;
use crate::common::parameters::Update;
use crate::common::Template;
use crate::common::{Errors, Resource, ResourceData};
use crate::controller::{Controller, RPCCaller};

use crate::{getters, group_getters, rpc_delete_method};

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

impl ResourceData for User {
    fn get_data(&self) -> &Resource {
        &self.resource
    }

    // TODO: return resource type ?
    //fn get_type(&self) -> ResourceType {
    //    ResourceType::User
    //}
    fn get_type(&self) -> &str {
        "USER"
    }
}

impl User {
    getters!("USER");
    group_getters!("USER");
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
    rpc_delete_method!(delete, "one.user.delete");

    pub fn info(&self) -> Result<User, Errors> {
        let resp_txt = self
            .controller
            .client
            .call("one.user.info", vec![self.id.into()])?;

        let body = self.controller.parse_body_resp(resp_txt)?;
        match Resource::from(&body) {
            Ok(resource) => Ok(User { resource }),
            Err(e) => Err(Errors::Roca(format!("Failed to parse the resource: {}", e))),
        }
    }

    pub fn update(&self, tpl: String, policy: Update) -> Result<(), Errors> {
        let resp_txt = self.controller.client.call(
            "one.user.update",
            vec![
                self.id.into(),
                tpl.into(),
                match policy {
                    Update::Replace => 0,
                    Update::Merge => 1,
                }
                .into(),
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
            vec![
                self.id.into(),
                name.into(),
                token.into(),
                period.into(),
                gid.into(),
            ],
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
                println!("user id: {}", infos.id().unwrap());
                println!("user name: {}", infos.name().unwrap());
                println!("user GID: {}", infos.gid().unwrap());
                println!("user GNAME: {}", infos.groupname().unwrap());
                println!(
                    "user AUTH_DRIVER: {}",
                    infos.get_str("AUTH_DRIVER").unwrap()
                );
            }
            _ => panic!("Error on user info"),
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

        let response = controller.users().allocate("test-alloc", "test-alloc", "");
        let user_id = match response {
            Ok(id) => id,
            _ => panic!("Error allocating the user"),
        };
        let ucontroller = controller.user(user_id);

        println!("User ID (test-rust): {}", user_id);

        assert!(user_id > 0);

        // Delete the user
        let response = ucontroller.delete();

        assert!(response.is_ok());
    }

    #[test]
    fn user_login() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2"),
        );
        let controller = Controller::new(client);

        // Create the user
        let name = "test-login";
        let response = controller.users().allocate(name, "password", "");

        let user_id = match response {
            Ok(id) => id,
            _ => panic!("Error allocating the user"),
        };

        let ucontroller = controller.user(user_id);

        // Test loging
        let response = ucontroller.login(name, "", 60, 0);

        match response {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };

        // Delete the user
        let response = ucontroller.delete();

        assert!(response.is_ok());
    }
}

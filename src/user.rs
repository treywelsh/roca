//! The user module allows to interact with OpenNebula users

use crate::common::getters::{ResourceInternal, ResourcePublic};
use crate::common::Template;
use crate::common::{Errors, Resource, ResourceData};
use crate::controller::{Controller, RPCCaller};

use crate::{getters, group_getters};

#[allow(dead_code)]
#[derive(Debug)] //
pub struct UserController<'a, C: RPCCaller> {
    pub controller: &'a Controller<C>,
    pub id: i32,
}

#[allow(dead_code)]
pub struct User {
    resource: Resource,
}

impl ResourceData for User {
    fn get_data(&self) -> &Resource {
        &self.resource
    }

    fn get_type(&self) -> &str {
        "USER"
    }
}

// add get_str, get_i64...
impl ResourceInternal for User {}

impl ResourcePublic for User {}

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

#[allow(dead_code)]
impl<'a, C: RPCCaller> UserController<'a, C> {
    // TODOs:
    // - enum form auth_drv options
    // - more helpers without some options (i.e groups, auth_drv)
    // - add last parameter: array of groups. Currently the client only support Array(Vec<Value>)
    pub fn allocate(&self, name: &str, passwd: &str, auth_drv: &str) -> Result<i32, String> {
        let req = self
            .controller
            .client
            .new_request("one.user.allocate")
            .arg(name)
            .arg(passwd)
            .arg(auth_drv);

        match self.controller.client.call(req) {
            Ok(resp) => {
                if resp.rc() {
                    if let Some(value) = resp.get_int(1) {
                        return Ok(value);
                    }
                } else if let Some(value) = resp.get_str(1) {
                    return Err(String::from(value));
                }
            }

            Err(e) => return Err(e),
        };

        Err(String::from(
            "The position required does not math the type.",
        ))
    }

    pub fn delete(self, id: i32) -> Result<(), String> {
        let req = self
            .controller
            .client
            .new_request("one.user.delete")
            .arg(id);
        let response = self.controller.client.call(req);

        match response {
            Ok(resp) => {
                if resp.rc() {
                    Ok(())
                } else {
                    match resp.get_str(1) {
                        Some(body) => Err(String::from(body)),
                        _ => Err(String::from(
                            "The position required does not math the type.",
                        )),
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn info(&self) -> Result<User, String> {
        let req = self
            .controller
            .client
            .new_request("one.user.info")
            .arg(self.id);
        let response = self.controller.client.call(req);

        match response {
            Ok(resp) => match resp.get_str(1) {
                Some(body) => {
                    if resp.rc() {
                        let resource = match Resource::from(body) {
                            Ok(r) => r,
                            Err(e) => return Err(format!("Failed to parse the resource: {}", e)),
                        };
                        let obj: User = User { resource };
                        Ok(obj)
                    } else {
                        Err(String::from(body))
                    }
                }
                _ => Err(String::from(
                    "The position required does not math the type.",
                )),
            },
            Err(e) => Err(e),
        }
    }

    pub fn passwd(&self, new_passd: i32) -> Result<(), String> {
        let req = self
            .controller
            .client
            .new_request("one.user.passwd")
            .arg(new_passd);
        let response = self.controller.client.call(req);

        match response {
            Ok(resp) => {
                if resp.rc() {
                    Ok(())
                } else {
                    match resp.get_str(1) {
                        Some(body) => Err(String::from(body)),
                        _ => Err(String::from(
                            "The position required does not math the type.",
                        )),
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn login(&self, name: &str, token: &str, period: i32, gid: i32) -> Result<String, String> {
        let req = self
            .controller
            .client
            .new_request("one.user.login")
            .arg(name)
            .arg(token)
            .arg(period)
            .arg(gid);
        let response = self.controller.client.call(req);

        match response {
            Ok(resp) => match resp.get_str(1) {
                Some(token) => {
                    if resp.rc() {
                        Ok(String::from(token))
                    } else {
                        Err(String::from(token))
                    }
                }
                _ => Err(String::from(
                    "The position required does not math the type.",
                )),
            },

            Err(e) => Err(e),
        }
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
        let user_controller = controller.user(0);

        let response = user_controller.allocate("test-alloc", "test-alloc", "");

        let user_id = match response {
            Ok(id) => id,
            _ => panic!("Error allocating the user"),
        };

        println!("User ID (test-rust): {}", user_id);

        assert!(user_id > 0);

        // Delete the user
        let response = user_controller.delete(user_id);

        assert_eq!(response, Ok(()));
    }

    #[test]
    fn user_login() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2"),
        );
        let controller = Controller::new(client);
        let user_controller = controller.user(0);

        // Create the user
        let name = "test-login";
        let response = user_controller.allocate(name, "password", "");

        let user_id = match response {
            Ok(id) => id,
            _ => panic!("Error allocating the user"),
        };

        let user_controller = controller.user(user_id);

        // Test loging
        let response = user_controller.login(name, "", 60, 0);

        match response {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };

        // Delete the user
        let response = user_controller.delete(user_id);

        assert_eq!(response, Ok(()));
    }
}

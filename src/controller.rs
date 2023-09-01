//use crate::client::Response;
use crate::common::Errors;
use crate::user::{UserController, UsersController};
use crate::vm::{VirtualMachineController, VirtualMachinesController};

use serde_xmlrpc::Value;

/// RPCCaller is a trait that a XML-RPC client should satisfy to allow the Roca controller to drive it
pub trait RPCCaller {
    fn call(&self, name: &str, args: Vec<Value>) -> Result<String, Errors>;
}

/// The Roca controller allow resource access in a hierachical way
#[derive(Debug)]
pub struct Controller<C: RPCCaller> {
    pub client: C,
}

impl<C: RPCCaller> Controller<C> {
    pub fn new(client: C) -> Self {
        Controller { client }
    }

    pub fn parse_id_resp(&self, raw: String) -> Result<i32, Errors> {
        let result = serde_xmlrpc::response_from_str::<(bool, i32)>(&raw);
        if let Err(e) = result {
            return Err(Errors::XMLRPC(e));
        }

        let (success, id) = result.unwrap();
        if success {
            Ok(id)
        } else {
            //TODO: handle last parameter error code

            let result = serde_xmlrpc::response_from_str::<(bool, String)>(&raw);
            if let Err(e) = result {
                return Err(Errors::XMLRPC(e));
            }
            let err = result.unwrap().1;

            Err(Errors::OpenNebula(err))
        }
    }

    pub fn parse_body_resp(&self, raw: String) -> Result<String, Errors> {
        let result = serde_xmlrpc::response_from_str::<(bool, String)>(&raw);
        if let Err(e) = result {
            return Err(Errors::XMLRPC(e));
        }

        //TODO: handle last parameter error code
        let (success, ret) = result.unwrap();
        if success {
            Ok(ret)
        } else {
            Err(Errors::OpenNebula(ret))
        }
    }

    pub fn parse_resp(&self, raw: String) -> Result<(), Errors> {
        let result = serde_xmlrpc::response_from_str::<(bool, String)>(&raw);
        if let Err(e) = result {
            return Err(Errors::XMLRPC(e));
        }

        //TODO: handle last parameter error code
        let (success, ret) = result.unwrap();
        if success {
            Ok(())
        } else {
            Err(Errors::OpenNebula(ret))
        }
    }

    pub fn user(&self, id: i32) -> UserController<C> {
        UserController::<C> {
            controller: self,
            id,
        }
    }

    pub fn users(&self) -> UsersController<C> {
        UsersController::<C> { controller: self }
    }

    pub fn virtual_machine(&self, id: i32) -> VirtualMachineController<C> {
        VirtualMachineController::<C> {
            controller: self,
            id,
        }
    }

    pub fn virtual_machines(&self) -> VirtualMachinesController<C> {
        VirtualMachinesController::<C> { controller: self }
    }
}

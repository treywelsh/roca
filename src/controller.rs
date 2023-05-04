//use crate::client::Response;
use crate::common::Errors;
use crate::user::UserController;

use serde_xmlrpc::Value;

pub trait RPCCaller {
    fn call(&self, name: &str, args: Vec<Value>) -> Result<(bool, String), Errors>;
}

#[derive(Debug)]
pub struct Controller<C: RPCCaller> {
    pub client: C,
}

impl<C: RPCCaller> Controller<C> {
    pub fn new(client: C) -> Self {
        Controller { client }
    }

    pub fn user(&self, id: i32) -> UserController<C> {
        UserController::<C> {
            controller: self,
            id,
        }
    }
}

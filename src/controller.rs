//use crate::client::Response;
use crate::common::Errors;
use crate::user::{UserController, UsersController};

use serde_xmlrpc::Value;

/// RPCCaller is a trait that a XML-RPC client should satisfy to allow the Roca controller to drive it
pub trait RPCCaller {
    fn call(&self, name: &str, args: Vec<Value>) -> Result<(bool, String), Errors>;
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

    pub fn user(&self, id: i32) -> UserController<C> {
        UserController::<C> {
            controller: self,
            id,
        }
    }

    pub fn users(&self) -> UsersController<C> {
        UsersController::<C> { controller: self }
    }
}

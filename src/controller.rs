use crate::client::{ClientXMLRPC, Response};
use crate::user::UserController;

use xmlrpc::Request;

pub trait RPCCaller {
    fn new_request<'a>(&self, name: &'a str) -> Request<'a>;
    fn call(&self, request: Request) -> Result<Response, String>;
}

#[derive(Debug)]
pub struct Controller<C: RPCCaller> {
    pub client: C,
}

impl<C: RPCCaller> Controller<C> {
    pub fn new(client: C) -> Self {
        Controller { client: client }
    }

    pub fn user(&self, id: i32) -> UserController<C> {
        UserController::<C> {
            controller: self,
            id: id,
        }
    }
}

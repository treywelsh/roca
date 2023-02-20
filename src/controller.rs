use crate::client::ClientXMLRPC;
use crate::user::UserController;

#[derive(Debug)]
pub struct Controller {
    pub client: ClientXMLRPC,
}

impl Controller {
    pub fn new(client: ClientXMLRPC) -> Self {
        Controller { client: client }
    }

    pub fn user(&self, id: i32) -> UserController {
        UserController {
            controller: self,
            id: id,
        }
    }
}

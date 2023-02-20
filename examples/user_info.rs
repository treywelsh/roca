extern crate roca;

use roca::client::ClientXMLRPC;
use roca::controller::Controller;

fn main() {
    let client = ClientXMLRPC::new(
        String::from("oneadmin:opennebula"),
        String::from("http://localhost:2633/RPC2"),
    );
    let controller = Controller::new(client);
    let user = controller.user(0);

    let uinfos = match user.info() {
        Ok(info) => info,
        Err(e) => panic!("Error on user info: {}", e),
    };

    println!("user infos: {:#?}", uinfos)
}

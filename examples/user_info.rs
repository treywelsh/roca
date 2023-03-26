extern crate roca;

use std::process::exit;

use roca::prelude::*;

fn main() {
    let client = ClientXMLRPC::new(
        String::from("oneadmin:pDi4mFBHue"),
        String::from("http://192.168.33.10:2633/RPC2"),
    );
    let controller = Controller::new(client);
    let user = controller.user(1);

    // TODO: shouldn't be mut... ?
    let uinfos = match user.info() {
        Ok(info) => info,
        Err(e) => {
            println!("Error on user info: {}", e);
            exit(1)
        }
    };

    println!("user id: {}", uinfos.id().unwrap());
    println!("user name: {}", uinfos.name().unwrap());
    println!("user GID: {}", uinfos.gid().unwrap());
    println!("user GNAME: {}", uinfos.groupname().unwrap());
    println!(
        "user get_str on AUTH_DRIVER: {}",
        uinfos.get_str("AUTH_DRIVER").unwrap()
    );
    println!(
        "user get_str on template TOKEN_PASSWORD: {}",
        uinfos.template().get_str("TOKEN_PASSWORD").unwrap()
    );
}

extern crate roca;

use std::process::exit;

use roca::{common::template_builder::TemplateBuilder, prelude::*};

fn main() {
    let client = ClientXMLRPC::new(
        String::from("oneadmin:pDi4mFBHue"),
        String::from("http://192.168.33.10:2633/RPC2"),
    );
    let controller = Controller::new(client);

    // create an user
    let uid = match controller.users().allocate("testuser", "testuser", "") {
        Ok(id) => id,
        Err(e) => {
            eprintln!("user allocation error: {}", e);
            exit(1);
        }
    };

    let ucontroller = controller.user(uid);

    // customize the user
    let mut tpl = TemplateBuilder::new();
    tpl.add_pair("tag1", "value1");
    tpl.add_pair("tag2", "value2");

    if let Err(e) = ucontroller.update(tpl.to_string()) {
        eprintln!("Error on user info: {}", e);
        exit(1)
    }

    // read the user informations

    let uinfos = match ucontroller.info() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error on user info: {}", e);
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

    // delete the user
    if let Err(e) = ucontroller.delete() {
        eprintln!("can't delete user ID:{}: {}", uid, e);
    }
}

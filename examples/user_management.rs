extern crate roca;

use std::process::exit;

use roca::prelude::*;

fn main() {
    let client = ClientXMLRPC::new(
        String::from("oneadmin:pDi4mFBHue"),
        String::from("http://192.168.33.10:2633/RPC2"),
    );
    let controller = Controller::new(client);

    // create an user
    println!("allocating an user...");

    let uid = match controller.users().allocate("testuser", "testuser", "") {
        Ok(id) => id,
        Err(e) => {
            eprintln!("user allocation error: {}", e);
            exit(1);
        }
    };

    let ucontroller = controller.user(uid);

    // read the user informations
    println!("retrieving user datas...");

    let uinfos = match ucontroller.info() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error on user info: {}", e);
            exit(1)
        }
    };

    println!("Fetch some datas via getters:");
    println!("  id: {}", uinfos.id().unwrap());
    println!("  name: {}", uinfos.name().unwrap());
    println!("  GID: {}", uinfos.gid().unwrap());
    println!("  GNAME: {}", uinfos.groupname().unwrap());
    println!(
        "  get_str on AUTH_DRIVER: {}",
        uinfos.get_str("AUTH_DRIVER").unwrap()
    );
    let template = uinfos.template();
    println!(
        "  get_str on template TOKEN_PASSWORD: {}\n",
        template.get_str("TOKEN_PASSWORD").unwrap()
    );
    println!("Full User XML content: {}", uinfos);

    // customize the user
    // we can build the template from scratch via a template builder
    let mut tpl = TemplateBuilder::new();

    // OR we're able to update existing template
    //let mut template_mut = uinfos.template_mut();

    tpl.put_str("tag1", "value1");
    tpl.put_str("tag2", "value2");

    //tpl.add_vector(Vector(
    //    "tag_vec1".to_string(),
    //    vec![
    //        Pair("tag3".to_string(), "value3".to_string()),
    //        Pair("tag4".to_string(), "value4".to_string()),
    //    ],
    //));

    println!("update user with template: {}\n", tpl);
    if let Err(e) = ucontroller.update(tpl, parameters::UpdateType::Merge) {
        eprintln!("Error on user update: {}", e);
    }

    // read the user informations after the update
    println!("retrieving up to date user datas...");

    let uinfos = match ucontroller.info() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error on user info: {}", e);
            exit(1)
        }
    };

    println!("Full User XML content: {}", uinfos);

    // TODO: try replace update and delete elements etc...

    // delete the user
    println!("deleting an user...");
    if let Err(e) = ucontroller.delete() {
        eprintln!("can't delete user ID:{}: {}", uid, e);
    }
}

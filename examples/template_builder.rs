extern crate roca;

use roca::prelude::*;

fn main() {
    // we can build the template from scratch via an empty resource
    let mut tpl = template::Builder::new();

    tpl.put_str("tag1", "value1");
    tpl.put_str("tag2", "value2");

    let xml = tpl.to_string();
    println!("template builder: {}\n", xml);
}

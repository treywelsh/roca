extern crate roca;

use roca::common::template_builder::{Pair, TemplateBuilder, Vector};

fn main() {
    // customize the user
    let mut tpl = TemplateBuilder::new();

    println!("add tag1 and tag2 pairs, and vec1 vector");
    tpl.add_pair("tag1", "value1");
    tpl.add_pair("tag2", "value2");
    tpl.add_vector(Vector(
        "vec1".to_string(),
        vec![
            Pair("tag3".to_string(), "value3".to_string()),
            Pair("tag4".to_string(), "value4".to_string()),
        ],
    ));

    println!("remove tag2 and add tag5");
    tpl.rm_pair("tag2");
    tpl.add_pair("tag5", "value5");

    println!("Template:\n{}\n", tpl);
}

//! An OpenNebula XML-RPC API binding for Rust.
//!
//! The `roca` crate provides a binding for interfacing [OpenNebula XML-RPC API].
//!
//! [OpenNebula XML-RPC API]: http://docs.opennebula.org/stable/integration/system_interfaces/api.html

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Pair(String, String);

#[derive(Debug, Clone)]
struct Vector(String, Vec<Pair>);

#[derive(Debug, Clone)]
pub struct Template {
    pairs: Vec<Pair>,
    vectors: Vec<Vector>,
}

impl Template {
    fn new() -> Self {
        Template {
            pairs: Vec::new(),
            vectors: Vec::new(),
        }
    }
}

//#[derive(Debug, Deserialize)]
//pub struct VM {
//    id: i64,
//    template: Template,
//}

struct TemplateVisitor;

impl<'de> Visitor<'de> for TemplateVisitor {
    type Value = Template;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = Template::new();

        while let Some(key) = access.next_key::<String>()? {
            let mut map_value = access.next_value::<HashMap<String, String>>().unwrap();

            let opt_value = map_value.remove("$text");
            if let Some(value) = opt_value {
                map.pairs.push(Pair(key, value));
            } else {
                let mut vector = Vec::new();
                for (k, v) in map_value {
                    vector.push(Pair(k, v))
                }
                map.vectors.push(Vector(key, vector));
            }
        }

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for Template {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TemplateVisitor {})
    }
}

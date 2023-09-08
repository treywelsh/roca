use std::rc::Rc;

use xml_doc::{Document, Element, ReadOptions};

use crate::common::Errors;

// TODO: implement methods ?

pub struct ResourcePool {
    pub document: Rc<Document>,
    pub root: Element,
}

// TODO: return an iter ?
pub fn build_pool(raw_xml: &str, resource_type: &str) -> Result<Vec<ResourcePool>, Errors> {
    let mut opts = ReadOptions::default();
    opts.require_decl = false;

    let document = match Document::parse_reader_with_opts(raw_xml.as_bytes(), opts) {
        Ok(p) => Rc::new(p),
        Err(e) => return Err(e.into()),
    };

    let mut resources = Vec::new();
    let root = document.root_element().unwrap();
    let elements = root.find_all(&document, resource_type);

    for element in elements {
        resources.push(ResourcePool {
            document: document.clone(),
            root: element,
        })
    }

    Ok(resources)
}

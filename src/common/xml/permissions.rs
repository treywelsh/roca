use xml_doc::{Document, Element};

use crate::common::Errors;

pub fn get_field(document: &Document, perm_elem: Element, name: &str) -> Result<u8, Errors> {
    let perm_field = match perm_elem.find(document, name) {
        Some(e) => e,
        None => return Err(Errors::NotFound(name.to_string())),
    };

    // this pair is not expected to have a childs
    if perm_field.children(document).len() > 1 {
        Err(Errors::HasChilds(name.to_string()))
    } else {
        Ok(perm_field.text_content(document).parse::<u8>()?)
    }
}

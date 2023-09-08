use xml_doc::{Document, Element, ReadOptions};

use crate::common::Errors;

/// ResourceGetter trait have to be implemented by each resource in order to enable specilization of default
/// methods defined by ResourceInternal and ResourcePublic traits
pub trait ResourceGetter {
    fn get_internal(&self) -> (&Document, &Element);
}

pub trait ResourceGetterMut {
    fn get_internal_mut(&mut self) -> (&mut Document, &mut Element);
}

// IS IT STILL USEFUL ?
pub struct Resource {
    pub document: Document,
    pub root: Element,
}

impl TryFrom<&str> for Resource {
    type Error = Errors;

    fn try_from(raw_xml: &str) -> Result<Self, Self::Error> {
        let mut opts = ReadOptions::default();
        opts.require_decl = false;

        let document = match Document::parse_reader_with_opts(raw_xml.as_bytes(), opts) {
            Ok(p) => p,
            Err(e) => return Err(e.into()),
        };
        let root = document.root_element().unwrap();

        Ok(Resource { document, root })
    }
}

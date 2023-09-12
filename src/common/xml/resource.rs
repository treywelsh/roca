use xml_doc::{Document, Element, ReadOptions};

use crate::common::Errors;

// TODO: make a macro declaring a resource
#[macro_export]
macro_rules! define_resource {
    ($resource_name:ident) => {
        use xml_doc::{Document, Element};
        use $crate::common::resource_getters::ResourceGetters;
        use $crate::common::xml::resource::{XMLDocGetters, XMLDocGettersMut};

        pub struct $resource_name {
            resource: Resource,
        }

        // read only
        impl XMLDocGetters for $resource_name {
            fn get_internal(&self) -> (&Document, &Element) {
                (&self.resource.document, &self.resource.root)
            }
        }

        // read-write
        impl XMLDocGettersMut for $resource_name {
            fn get_internal_mut(&mut self) -> (&mut Document, &mut Element) {
                (&mut self.resource.document, &mut self.resource.root)
            }
        }

        impl ResourceGetters for $resource_name {}

        impl Display for $resource_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.resource.document.write_str().unwrap())
            }
        }
    };
}

/// XMLDocGetters trait have to be implemented by each resource in order to enable specialized methods
/// defined by BaseGetters trait
pub trait XMLDocGetters {
    fn get_internal(&self) -> (&Document, &Element);
}

pub trait XMLDocGettersMut {
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

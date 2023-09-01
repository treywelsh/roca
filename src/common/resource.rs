use xml_doc::{Document, Element, ReadOptions};

use crate::common::Errors;

/// ResourceGetter trait have to be implemented by each resource in order to enable specilization of default
/// methods defined by ResourceInternal and ResourcePublic traits
pub trait ResourceGetter {
    fn get_resource(&self) -> &Resource;
    fn get_resource_mut(&mut self) -> &mut Resource;
}

pub struct Resource {
    pub document: Document,
    pub root: Element,
    //pub template: Element,
}

impl Resource {
    /// new returns an empty resource useful to build
    pub fn new() -> Self {
        let mut document = Document::new();
        let root = Element::new(&mut document, "TEMPLATE");
        document.push_root_node(root.as_node());
        //let template = Element::new(&mut document, "TEMPLATE");
        //template.push_to(&mut document, root);
        Resource {
            document,
            root,
            //template,
        }
    }

    // TODO: implement a trait?
    pub fn from(raw_xml: &str) -> Result<Resource, Errors> {
        let mut opts = ReadOptions::default();
        opts.require_decl = false;

        let document = match Document::parse_reader_with_opts(raw_xml.as_bytes(), opts) {
            Ok(p) => p,
            Err(e) => return Err(e.into()),
        };
        let root = document.root_element().unwrap();
        //let template = root.find(&document, "TEMPLATE").unwrap();
        Ok(Resource {
            document,
            root,
            //template,
        })
    }

    pub fn id(&self) -> Result<i64, Errors> {
        self.get_i64(&self.root, "ID")
    }

    pub fn name(&self) -> Result<String, Errors> {
        self.get_str(&self.root, "NAME")
    }

    pub fn get_str(&self, element: &Element, name: &str) -> Result<String, Errors> {
        let found = match element.find(&self.document, name) {
            Some(e) => e,
            None => return Err(Errors::NotFound(name.to_string())),
        };
        if found.children(&self.document).len() > 1 {
            Err(Errors::HasChilds(name.to_string()))
        } else {
            Ok(found.text_content(&self.document))
        }
    }

    pub fn get_i64(&self, element: &Element, name: &str) -> Result<i64, Errors> {
        let i_str = self.get_str(element, name)?;

        Ok(i_str.parse::<i64>()?)
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self::new()
    }
}

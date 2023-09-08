use std::fmt::Display;

use xml_doc::{Document, Element};

use crate::common::errors::Errors;
use crate::common::template_elements::Vector;
use crate::common::template_getters::{TemplateCommonGetters, TemplateGetter};

/// Allow to build a template from scratch
pub struct TemplateBuilder {
    document: Document,
    element: Element,
}

impl<'a> TemplateGetter<'a> for TemplateBuilder {
    fn get_document(&self) -> &Document {
        &self.document
    }

    fn get_element(&self) -> &Element {
        &self.element
    }
}

impl<'a> TemplateCommonGetters<'a> for TemplateBuilder {}

impl TemplateBuilder {
    pub fn new() -> Self {
        let mut document = Document::new();
        let element = Element::new(&mut document, "TEMPLATE");
        document.push_root_node(element.as_node());

        TemplateBuilder { document, element }
    }

    pub fn put_str(&mut self, name: &str, value: &str) {
        Element::build(name)
            .text_content(value)
            .push_to(&mut self.document, self.element);
    }

    pub fn put_vector(&mut self, vec: Vector) {
        let element = Element::build(vec.0).push_to(&mut self.document, self.element);
        for p in vec.1 {
            Element::build(p.0)
                .text_content(p.1)
                .push_to(&mut self.document, element);
        }
    }

    //pub fn get_vector()

    /// Remove all pairs with key "name"
    pub fn rm(&mut self, name: &str) -> Result<(), Errors> {
        let elements = self.element.find_all(&self.document, name);
        if elements.is_empty() {
            return Err(Errors::Template(format!(
                "can't remove {} from template: not found",
                name,
            )));
        }

        for e in elements {
            e.detatch(&mut self.document)?;
        }

        Ok(())
    }
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for TemplateBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.document.write_str().unwrap())
    }
}

#[cfg(test)]
mod builder {
    use crate::{
        common::template_elements::Vector, common::template_getters::TemplateCommonGetters,
        prelude::TemplateBuilder,
    };

    #[test]
    fn add_retrieve_pairs() {
        let mut tpl = TemplateBuilder::new();

        tpl.put_str("tag1", "value1");
        tpl.put_str("tag2", "value2");

        let tag1 = tpl.get_str("tag1");

        // retrieve first value
        assert!(tag1.is_ok());
        assert_eq!(tag1.unwrap(), "value1");
    }

    #[test]
    fn add_delete_pairs() {
        let mut tpl = TemplateBuilder::new();

        tpl.put_str("tag1", "value1");
        tpl.put_str("tag1", "value2");

        // remove all elements
        tpl.rm("tag1");

        let tag1 = tpl.get_str("tag1");
        print!("{:?}", tag1);
        assert!(tag1.is_err());

        tpl.put_str("tag1", "value3");
        let tag1 = tpl.get_str("tag1");
        assert_eq!(tag1.unwrap(), "value3");
    }

    #[test]
    fn add_delete_vector() {
        let mut tpl = TemplateBuilder::new();

        tpl.put_str("tag1", "value1");

        let mut vec = Vector::new("vec1");
        vec.put_str("key", "value");
        vec.put_str("key2", "value2");

        tpl.put_vector(vec);

        // retrieve vector
        let res = tpl.get_vector("vec1");
        assert!(res.is_ok());
        let vec1 = res.unwrap();

        // retrieve pair from vector
        let pair2 = vec1.get_str("key2");
        assert!(pair2.is_ok());
        assert_eq!(pair2.unwrap(), "value2");

        let pair1 = vec1.get_str("key");
        assert!(pair1.is_ok());
        assert_eq!(pair1.unwrap(), "value");

        let mut vec1 = vec1;

        // remove pair
        vec1.rm("key");
        let pair1 = vec1.get_str("key");
        assert!(pair1.is_err());

        // remove vector
        tpl.rm("vec1");
        let res = tpl.get_vector("vec1");
        assert!(res.is_err());
    }
}

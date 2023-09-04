use std::fmt::Display;
use std::str::FromStr;

use xml_doc::{Document, Element};

use crate::common::errors::Errors;
use crate::common::template_getters::TemplateCommonGetters;
use crate::common::template_getters::TemplateGetter;

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

    // How to put a str inside of a vector ? Replace the whole vector ? if there's several vectors, replace all ?
    pub fn put_str(&mut self, name: &str, value: &str) {
        Element::build(name)
            .text_content(value)
            .push_to(&mut self.document, self.element);
    }

    /// Delete all pairs with key "name"
    pub fn del(&mut self, name: &str) -> Result<(), Errors> {
        let elements = self.element.find_all(&self.document, name);
        if elements.is_empty() {
            return Err(Errors::Template(format!(
                "can't delete {} from template: not found",
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
        common::permissions::{flags, Permissions},
        prelude::{TemplateBuilder, TemplateCommonGetters},
    };

    #[test]
    fn add_retrieve_elements() {
        let mut tpl = TemplateBuilder::new();

        tpl.put_str("tag1", "value1");
        tpl.put_str("tag2", "value2");

        let tag1 = tpl.get_str("tag1");

        // retrieve first value
        assert!(tag1.is_ok());
        assert_eq!(tag1.unwrap(), "value1");
    }

    #[test]
    fn add_delete_elements() {
        let mut tpl = TemplateBuilder::new();

        tpl.put_str("tag1", "value1");
        tpl.put_str("tag1", "value2");

        // delete all elements
        tpl.del("tag1");

        let tag1 = tpl.get_str("tag1");
        print!("{:?}", tag1);
        assert!(tag1.is_err());

        tpl.put_str("tag1", "value3");
        let tag1 = tpl.get_str("tag1");
        assert_eq!(tag1.unwrap(), "value3");
    }
}

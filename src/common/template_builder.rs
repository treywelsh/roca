use std::fmt::Display;

use xml_doc::{Document, Element};

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
}

impl Display for TemplateBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.document.write_str().unwrap())
    }
}

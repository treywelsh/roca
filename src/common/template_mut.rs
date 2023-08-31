use xml_doc::{Document, Element};

use crate::common::template_getters::TemplateCommonGetters;
use crate::common::template_getters::TemplateGetter;

use super::Errors;

pub struct TemplateMut<'a> {
    document: &'a mut Document,
    element: Element,
}

impl<'a> TemplateMut<'a> {
    pub fn from_resource(document: &'a mut Document, element: Element) -> Self {
        TemplateMut { document, element }
    }
}

impl<'a> TemplateGetter<'a> for TemplateMut<'a> {
    fn get_document(&'a self) -> &'a Document {
        self.document
    }

    fn get_element(&'a self) -> &'a Element {
        &self.element
    }
}

impl<'a> TemplateCommonGetters<'a> for TemplateMut<'a> {}

impl<'a> TemplateMut<'a> {
    // TODO:
    // add: vector, i64, float
    // allow to delete pair and vector
    // allow to add a pair inside of a vector
    pub fn put_str(&mut self, name: &str, value: &str) {
        Element::build(name)
            .text_content(value)
            .push_to(self.document, self.element);
    }

    pub fn del(&mut self, name: &str) -> Result<(), Errors> {
        let e = self.element.find(self.document, name).unwrap();

        Ok(e.detatch(self.document)?)
    }
}

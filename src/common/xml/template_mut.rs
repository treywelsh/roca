use std::fmt::Display;

use xml_doc::{Document, Element};

use crate::common::resource_getters::Get;
use crate::common::template::elements::Vector;
use crate::common::xml::resource::XMLDocGetters;
use crate::common::Errors;

/// Allow to access and mutate the ressource template attributes
pub struct TemplateMut<'a> {
    document: &'a mut Document,
    element: Element,
}

impl<'a> TemplateMut<'a> {
    pub fn from_resource(document: &'a mut Document, element: Element) -> Self {
        TemplateMut { document, element }
    }
}

impl<'a> XMLDocGetters for TemplateMut<'a> {
    fn get_internal(&self) -> (&Document, &Element) {
        (self.document, &self.element)
    }
}

impl<'a> Display for TemplateMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.document.write_str().unwrap())
    }
}

// implement trait from blanket implementation methods
impl<'a> Get for TemplateMut<'a> {
    fn get(&self, key: &str) -> Result<String, crate::common::Errors> {
        crate::common::xml::shared_getters::BaseGetters::get(self, key)
    }

    fn get_vector(
        &self,
        key: &str,
    ) -> Result<crate::common::template::elements::Vector, crate::common::Errors> {
        crate::common::xml::shared_getters::BaseGetters::get_vector(self, key)
    }
}

impl<'a> TemplateMut<'a> {
    // TODO:
    // add: i64, float
    // allow to delete pair and vector
    // allow to add a pair inside of a vector
    pub fn put_str(&mut self, name: &str, value: &str) {
        Element::build(name)
            .text_content(value)
            .push_to(self.document, self.element);
    }

    pub fn put_vector(&mut self, vec: Vector) {
        let element = Element::build(vec.0).push_to(self.document, self.element);
        for p in vec.1 {
            Element::build(p.0)
                .text_content(p.1)
                .push_to(self.document, element);
        }
    }

    /// Remove all pairs with key "name"
    pub fn rm(&mut self, name: &str) -> Result<(), Errors> {
        let elements = self.element.find_all(self.document, name);
        if elements.is_empty() {
            return Err(Errors::Template(format!(
                "can't remove {} from template: not found",
                name,
            )));
        }

        for e in elements {
            e.detatch(self.document)?;
        }

        Ok(())
    }
}

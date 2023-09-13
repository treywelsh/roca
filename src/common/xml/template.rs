use std::fmt::Display;

use crate::common::{resource_getters::Get, template::elements::Pair};
use xml_doc::{Document, Element};

use crate::common::xml::resource::XMLDocGetters;

// there's a lifetime because we inherit of a reference
// to the document, coming from the resource structure
/// Allow to access the resource template attributes
pub struct Template<'a> {
    document: &'a Document, // it's ok when reading only, but what about writing ?
    element: Element,
}

// TODO: add methods:
// - gets (in case there is several pairs with the same key)
// - get_i64s ?
// - get_float64 (there is some CPU key with a floating type)
impl<'a> Template<'a> {
    //pub fn from_resource(document: &'a Document, element: Element) -> Self {
    //    Template { document, element }
    //}

    pub fn from_resource(document: &'a Document, element: Element) -> Self {
        Template { document, element }
    }
}

impl<'a> XMLDocGetters for Template<'a> {
    fn get_internal(&self) -> (&Document, &Element) {
        (self.document, &self.element)
    }
}

impl<'a> Display for Template<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.document.write_str().unwrap())
    }
}

// implement trait from blanket implementation methods
impl<'a> Get for Template<'a> {
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

// TODO: allow to iter on pairs and vectors
pub struct TemplatePairs {}

impl<'a> IntoIterator for Template<'a> {
    type Item = Pair;
    type IntoIter = TemplatePairs;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl Iterator for TemplatePairs {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

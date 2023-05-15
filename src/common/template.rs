use std::fmt::Display;

use xml_doc::{Document, Element};

use crate::common::template_elements::Pair;
use crate::common::template_getters::TemplateCommonGetters;
use crate::common::template_getters::TemplateGetter;

//use crate::common::template_getters::TemplateGetter;

// there's a lifetime because we inherit of a reference
// to the document, coming from the resource structure
pub struct Template<'a> {
    pub document: &'a Document, // it's ok when reading only, but what about writing ?
    pub element: Element,
}

// TODO: add methods:
// - get_strs (in case there is several pairs with the same key)
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

impl<'a> TemplateGetter<'a> for Template<'a> {
    fn get_document(&'a self) -> &'a Document {
        self.document
    }

    fn get_element(&'a self) -> &'a Element {
        &self.element
    }
}

impl<'a> TemplateCommonGetters<'a> for Template<'a> {}

//impl<'a> Display for Template<'a> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        // try to display only the template part of the XML
//        let mut document = Document::new();
//        document.push_root_node(self.element.as_node());
//        //f.write_str(&self.document.write_str().unwrap())
//        f.write_str(&document.write_str().unwrap())
//    }
//}

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

use std::fmt::Display;

use crate::common::{
    template::elements::Pair,
    template::{
        builder::{self, Builder},
        elements::Vector,
    },
};
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

    // TODO: replace by TryFrom ?
    pub fn from_resource(document: &'a Document, element: Element) -> Self {
        Template { document, element }
    }
}

impl<'a> From<Template<'a>> for Builder {
    fn from(template: Template<'a>) -> Self {
        let mut builder = Builder::new();

        for element in template.element.child_elements(template.document) {
            let childs = element.child_elements(template.document);
            if childs.is_empty() {
                // it's a pair
                builder.put_str(
                    element.name(template.document),
                    &element.text_content(template.document),
                );
            } else {
                // it's a vector
                let mut vec = Vector::new(element.name(template.document));
                for sub_element in element.child_elements(template.document) {
                    vec.1.push(Pair(
                        sub_element.name(template.document).to_string(),
                        sub_element.text_content(template.document),
                    ));
                }
                builder.put_vector(vec);
            }
        }

        builder
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

use crate::common::errors::Errors;
use crate::common::template::elements::{Pair, Vector};
use crate::common::xml::resource::XMLDocGetters;

// This file keep getters shared among resources (VM, User...) and templates

// blanket implementations
impl<T> BaseGetters for T where T: XMLDocGetters {}

/// Add default getters to work with dynamic XML content
pub trait BaseGetters: XMLDocGetters {
    fn get_str(&self, key: &str) -> Result<String, Errors> {
        let (document, element) = self.get_internal();
        let found = match element.find(document, key) {
            Some(e) => e,
            None => return Err(Errors::NotFound(key.to_string())),
        };
        if found.children(document).len() > 1 {
            Err(Errors::HasChilds(key.to_string()))
        } else {
            let mut buf = String::new();
            Ok(found.text_content(document))
        }
    }

    fn get_i64(&self, key: &str) -> Result<i64, Errors> {
        let i_str = self.get_str(key)?;

        Ok(i_str.parse::<i64>()?)
    }

    fn get_vector(&self, key: &str) -> Result<Vector, Errors> {
        let (document, element) = self.get_internal();

        let element = match element.find(document, key) {
            Some(e) => e,
            None => return Err(Errors::NotFound(key.to_string())),
        };

        let mut pairs = Vec::new();
        for sub_element in element.child_elements(document) {
            pairs.push(Pair(
                sub_element.name(document).to_string(),
                sub_element.text_content(document),
            ));
        }
        Ok(Vector(key.into(), pairs))
    }

    fn get_vectors(&self, key: &str) -> Result<Vec<Vector>, Errors> {
        let (document, element) = self.get_internal();

        let mut vectors = Vec::new();

        for element in element.find_all(document, key) {
            let mut pairs = Vec::new();
            for sub_element in element.child_elements(document) {
                // sub_element is not expected to have childs
                //if sub_element.children(self.document).len() > 1 {
                //    Err(Errors::HasChilds(name.to_string()))
                //} else {
                pairs.push(Pair(
                    sub_element.name(document).to_string(),
                    sub_element.text_content(document),
                ));
                //}
            }
            vectors.push(Vector(element.name(document).to_string(), pairs));
        }

        Ok(vectors)
    }
}

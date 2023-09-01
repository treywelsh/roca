use xml_doc::{Document, Element};

use crate::common::errors::Errors;
use crate::common::template_elements::{Pair, Vector};

pub trait TemplateGetter<'a> {
    fn get_document(&self) -> &Document;
    fn get_element(&self) -> &Element;
}

pub trait TemplateCommonGetters<'a>: TemplateGetter<'a> {
    // TODO: reuse methods from resource ?
    fn get_str(&self, name: &str) -> Result<String, Errors> {
        let tpl_element = self.get_element();
        let tpl_document = self.get_document();

        let element = match tpl_element.find(tpl_document, name) {
            Some(e) => e,
            None => return Err(Errors::NotFound(name.to_string())),
        };

        // this pair is not expected to have a childpub
        if element.children(tpl_document).len() > 1 {
            Err(Errors::HasChilds(name.to_string()))
        } else {
            Ok(element.text_content(tpl_document))
        }
    }

    fn get_i64(&self, name: &str) -> Result<i64, Errors> {
        let value_str = self.get_str(name)?;

        let v = value_str.parse()?;
        Ok(v)
    }

    fn get_vectors(&'a self, name: &str) -> Result<Vec<Vector>, Errors> {
        let tpl_element = self.get_element();
        let tpl_document = self.get_document();
        let mut vectors = Vec::new();

        for element in tpl_element.find_all(tpl_document, name) {
            let mut pairs = Vec::new();
            for sub_element in element.child_elements(tpl_document) {
                // sub_element is not expected to have childs
                //if sub_element.children(self.document).len() > 1 {
                //    Err(Errors::HasChilds(name.to_string()))
                //} else {
                pairs.push(Pair(
                    sub_element.name(tpl_document).to_string(),
                    sub_element.text_content(tpl_document),
                ));
                //}
            }
            vectors.push(Vector(element.name(tpl_document).to_string(), pairs));
        }

        Ok(vectors)
    }

    fn get_vector(&'a self, name: &str) -> Result<Vec<Pair>, Errors> {
        let tpl_element = self.get_element();
        let tpl_document = self.get_document();

        let element = match tpl_element.find(tpl_document, name) {
            Some(e) => e,
            None => return Err(Errors::NotFound(name.to_string())),
        };

        let mut pairs = Vec::new();
        for sub_element in element.child_elements(tpl_document) {
            pairs.push(Pair(
                sub_element.name(tpl_document).to_string(),
                sub_element.text_content(tpl_document),
            ));
        }
        Ok(pairs)
    }
}

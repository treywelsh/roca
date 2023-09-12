use std::fmt::Display;

use xml_doc::{Document, Element};

use crate::common::errors::Errors;
use crate::common::resource_getters::Get;
use crate::common::template::elements::{Pair, Vector};

// TODO:
// rename template_fmt to template
// add builder and elements to this directory
// then make template builder independant from XML, then add a conversion method
// -> this would allow to build custom format ?
// remove template_buider file

/// Allow to build a template from scratch
#[derive(Debug)]
pub struct Builder {
    pairs: Vec<Pair>,
    vectors: Vec<Vector>,
}

impl Get for Builder {
    fn get_vector(&self, key: &str) -> Result<Vector, Errors> {
        for v in &self.vectors {
            if v.0 == key {
                return Ok((*v).clone());
            }
        }
        Err(Errors::NotFound(
            "template builder: element not found".to_string(),
        ))
    }

    fn get_str(&self, key: &str) -> Result<String, Errors> {
        for p in &self.pairs {
            if p.0 == key {
                return Ok(p.1.clone());
            }
        }
        Err(Errors::NotFound(
            "template builder: element not found".to_string(),
        ))
    }

    fn get_i64(&self, key: &str) -> Result<i64, Errors> {
        let value_str = self.get_str(key)?;

        let v = value_str.parse()?;
        Ok(v)
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            pairs: Vec::new(),
            vectors: Vec::new(),
        }
    }

    pub fn put_str(&mut self, key: &str, value: &str) {
        self.pairs.push(Pair::new(key, value))
    }

    pub fn put_vector(&mut self, vec: Vector) {
        self.vectors.push(vec);
    }

    /// Remove all elements with key "name"
    pub fn rm(&mut self, key: &str) -> Result<(), Errors> {
        // remove pairs
        let pairs_len = self.pairs.len();
        self.pairs.retain(|p| p.0 != key);

        // remove vec
        let vecs_len = self.vectors.len();
        self.vectors.retain(|p| p.0 != key);

        if pairs_len == self.pairs.len() && vecs_len == self.vectors.len() {
            return Err(Errors::Template(format!(
                "can't remove {} from template builder: not found",
                key,
            )));
        }

        Ok(())
    }

    pub fn generate_xml(&self) -> Result<String, Errors> {
        let mut document = Document::new();
        let element = Element::new(&mut document, "TEMPLATE");
        let _ = document.push_root_node(element.as_node());

        for p in &self.pairs {
            Element::build(p.0.clone())
                .text_content(p.1.clone())
                .push_to(&mut document, element);
        }

        for vec in &self.vectors {
            let element = Element::build(vec.0.clone()).push_to(&mut document, element);
            for p in &vec.1 {
                Element::build(p.0.clone())
                    .text_content(p.1.clone())
                    .push_to(&mut document, element);
            }
        }

        Ok(document.write_str()?)
    }

    pub fn generate_onefmt(self) {}
}

impl Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.generate_xml().unwrap())
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use crate::common::resource_getters::Get;
    use crate::common::template::builder::{Builder, Vector};

    #[test]
    fn add_retrieve_pairs() {
        let mut tpl = Builder::new();

        tpl.put_str("tag1", "value1");
        tpl.put_str("tag2", "value2");

        let tag1 = tpl.get_str("tag1");

        // retrieve first value
        assert!(tag1.is_ok());
        assert_eq!(tag1.unwrap(), "value1");
    }

    #[test]
    fn add_delete_pairs() {
        let mut tpl = Builder::new();

        tpl.put_str("tag1", "value1");
        tpl.put_str("tag1", "value2");

        // remove all elements
        let _ = tpl.rm("tag1");

        let tag1 = tpl.get_str("tag1");
        print!("{:?}", tag1);
        assert!(tag1.is_err());

        tpl.put_str("tag1", "value3");
        let tag1 = tpl.get_str("tag1");
        assert_eq!(tag1.unwrap(), "value3");
    }

    #[test]
    fn add_delete_vector() {
        let mut tpl = Builder::new();

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

        assert_eq!(
            tpl.generate_xml().unwrap(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<TEMPLATE>\n  <tag1>value1</tag1>\n  <vec1>\n    <key>value</key>\n    <key2>value2</key2>\n  </vec1>\n</TEMPLATE>"
        );

        let mut vec1 = vec1;

        // remove pair
        let _ = vec1.rm("key");
        let pair1 = vec1.get_str("key");
        assert!(pair1.is_err());

        // remove vector
        let _ = tpl.rm("vec1");
        let res = tpl.get_vector("vec1");
        assert!(res.is_err());
    }
}

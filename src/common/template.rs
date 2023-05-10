use sxd_document::Package;
use sxd_xpath::{evaluate_xpath, Value};

use crate::common::errors::Errors;
use crate::common::template_builder::{Pair, TemplateBuilder, Vector};

pub struct Template<'a> {
    package: &'a Package,
    prefix_path: String,
}

impl<'a> Template<'a> {
    pub fn new(package: &'a Package, resource_type: &str) -> Self {
        let mut prefix_path = String::from("/");
        prefix_path.push_str(resource_type);
        prefix_path.push_str("/TEMPLATE/");

        Template {
            package,
            prefix_path,
        }
    }

    // TODO: pub fn get_strs(&self, name: &str) -> Result<Vec<String>, Errors> {)

    pub fn get_str(&self, name: &str) -> Result<String, Errors> {
        let mut path = self.prefix_path.clone();
        path.push_str(name);

        let document = self.package.as_document();
        let values = evaluate_xpath(&document, &path)?;

        if let Value::Nodeset(nodes) = values {
            let mut nodes = nodes.document_order();

            if nodes.len() > 1 {
                return Err(Errors::SeveralNodes(name.to_string()));
            } else if nodes.is_empty() {
                return Err(Errors::NotFound(name.to_string()));
            }
            let node = nodes.pop().unwrap();

            // TODO: avoid key allocation ?
            Ok(node.string_value())
        } else {
            Err(Errors::NotFound(name.to_string()))
        }
    }

    pub fn get_i64(&self, name: &str) -> Result<i64, Errors> {
        let value_str = self.get_str(name)?;

        let v = value_str.parse()?;
        Ok(v)
    }

    pub fn get_vectors(&self, name: &str) -> Result<Vec<Vector>, Errors> {
        let mut path = self.prefix_path.clone();
        path.push_str(name);

        let document = self.package.as_document();
        let mut vectors = Vec::new();

        let values = evaluate_xpath(&document, &path)?;

        if let Value::Nodeset(nodes) = values {
            for node in nodes.document_order() {
                // fill the vector
                let mut elements = Vec::new();
                for node in node.children() {
                    if node.text().is_some() {
                        continue;
                    }

                    let name = node.expanded_name().unwrap().local_part();
                    elements.push(Pair(name.to_string(), node.string_value()))
                }
                vectors.push(Vector(name.to_string(), elements));
            }
            Ok(vectors)
        } else {
            Err(Errors::NotFound(name.to_string()))
        }
    }

    pub fn get_vector(&self, name: &str) -> Result<Vec<Pair>, Errors> {
        let mut path = self.prefix_path.clone();
        path.push_str(name);

        let document = self.package.as_document();

        let values = evaluate_xpath(&document, &path)?;

        if let Value::Nodeset(nodes) = values {
            let mut nodes = nodes.document_order();
            if nodes.len() > 1 {
                return Err(Errors::SeveralNodes(name.to_string()));
            } else if nodes.is_empty() {
                return Err(Errors::NotFound(name.to_string()));
            }
            let node = nodes.pop().unwrap();

            // fill the vector
            let mut elements = Vec::new();
            for node in node.children() {
                if node.text().is_some() {
                    continue;
                }

                let name = node.expanded_name().unwrap().local_part();
                elements.push(Pair(name.to_string(), node.string_value()))
            }

            // TODO: never tested the iterator version
            //let elements: Vec<Pair> = node
            //    .children()
            //    .iter()
            //    .filter(|&node| node.text().is_some())
            //    .map(|node| {
            //        Pair(
            //            node.expanded_name().unwrap().local_part().to_string(),
            //            node.string_value(),
            //        )
            //    })
            //    .collect();

            Ok(elements)
        } else {
            Err(Errors::NotFound(name.to_string()))
        }
    }

    pub fn to_builder(&self) -> Result<TemplateBuilder, Errors> {
        let document = self.package.as_document();

        // remove trailing '/':
        let path = &self.prefix_path[..self.prefix_path.len() - 1];
        let values = evaluate_xpath(&document, path)?;

        let mut builder = TemplateBuilder::new();
        if let Value::Nodeset(node) = values {
            let tpl_node = node.document_order().pop().unwrap();

            for node in tpl_node.children() {
                let name = node.expanded_name().unwrap().local_part();

                if node.children().len() > 1 {
                    // it's a vec
                    let mut vector = Vector::new(name.to_string());
                    for n in node.children() {
                        let name = n.expanded_name().unwrap().local_part();

                        vector.add_pair(name.to_string(), n.string_value());
                    }
                    builder.add_vector(vector);
                } else {
                    builder.add_pair(name.to_string(), node.string_value());
                }
            }
        }

        Ok(builder)
    }
}

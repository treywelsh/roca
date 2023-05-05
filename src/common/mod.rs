use sxd_document::{parser, Package};
use sxd_xpath::evaluate_xpath;

mod errors;
pub use errors::Errors;

mod template;
pub use template::Template;

pub mod template_builder;

pub mod getters;
pub mod resource_methods;

// Resource add some methods to get XML attributes wrapping the usage of the sxd_xxx crates
pub struct Resource {
    package: Package,
    //document: Document<'a>,
    //path: Vec<String>, // TODO: replace by vec of strings ?
}

impl Resource {
    pub fn from(raw_xml: &str) -> Result<Resource, Errors> {
        let package = match parser::parse(raw_xml) {
            Ok(p) => p,
            Err(e) => return Err(e.into()),
        };
        Ok(Resource {
            package,
            //path: vec![format!("/{}", resource_type)],
            //document: package.as_document(),
        })
    }

    pub fn id(&self, resource_type: &str) -> Result<i64, Errors> {
        // TODO: remove alloc via kind of pool ?
        let mut path = String::from("/");
        path.push_str(resource_type);
        path.push_str("/ID");
        let res = match evaluate_xpath(&self.package.as_document(), &path) {
            Ok(id) => Ok(id.number() as i64),
            Err(e) => return Err(e.into()),
        };
        res
    }

    pub fn name(&self, resource_type: &str) -> Result<String, Errors> {
        // TODO: remove alloc via kind of pool ?
        let mut path = String::from("/");
        path.push_str(resource_type);
        path.push_str("/NAME");

        let document = self.package.as_document();
        let value = evaluate_xpath(&document, &path)?;

        Ok(value.into_string())
    }

    // get a vm pair (at root, not in the template)
    pub fn _get_str(&self, full_path: &str) -> Result<String, Errors> {
        let document = self.package.as_document();
        let value = evaluate_xpath(&document, full_path)?;

        Ok(value.into_string())
    }

    // get a vm pair (at root, not in the template)
    pub fn get_str(&self, resource_type: &str, name: &str) -> Result<String, Errors> {
        // TODO: remove alloc via kind of pool ?
        let mut path = String::from("/");
        path.push_str(resource_type);
        path.push('/');
        path.push_str(name);

        let document = self.package.as_document();
        let value = evaluate_xpath(&document, &path)?;

        Ok(value.into_string())
    }

    pub fn _get_i64(&self, full_path: &str) -> Result<i64, Errors> {
        let res = match evaluate_xpath(&self.package.as_document(), full_path) {
            Ok(id) => Ok(id.number() as i64),
            Err(e) => return Err(e.into()),
        };
        res
    }

    // TODO: remove mut
    pub fn get_i64(&self, resource_type: &str, name: &str) -> Result<i64, Errors> {
        let mut path = String::from("/");
        path.push_str(resource_type);
        path.push('/');
        path.push_str(name);

        let res = match evaluate_xpath(&self.package.as_document(), &path) {
            Ok(id) => Ok(id.number() as i64),
            Err(e) => return Err(e.into()),
        };
        res
    }
}

/// ResourceData trait have to be implemented by each resource in order to enable specilization of default
/// methods defined by ResourceInternal and ResourcePublic traits
pub trait ResourceData {
    fn get_data(&self) -> &Resource;
    fn get_type(&self) -> &str;
}

use crate::common::errors::Errors;
use crate::common::resource::ResourceGetter;

use crate::common::template::Template;
use crate::common::template_mut::TemplateMut;

impl<T> CommonGetters for T where T: ResourceGetter {}

// TODO: rename ?
pub trait CommonGetters: ResourceGetter {
    fn id(&self) -> Result<i64, Errors> {
        self.get_resource().id()
    }

    fn name(&self) -> Result<String, Errors> {
        self.get_resource().name()
    }

    fn get_str(&self, name: &str) -> Result<String, Errors> {
        self.get_resource().get_str(&self.get_resource().root, name)
    }

    fn get_i64(&self, name: &str) -> Result<i64, Errors> {
        self.get_resource().get_i64(&self.get_resource().root, name)
    }

    fn template(&self) -> Template {
        let document = &self.get_resource().document;
        let template = self.get_resource().root.find(document, "TEMPLATE").unwrap();

        Template::from_resource(document, template)
    }

    fn template_mut(&mut self) -> TemplateMut {
        let resource = self.get_resource_mut();
        let template = resource.root.find(&resource.document, "TEMPLATE").unwrap();

        TemplateMut::from_resource(&mut resource.document, template)
    }
}

pub trait Owner: ResourceGetter {
    fn uid(&self) -> Result<String, Errors> {
        self.get_resource()
            .get_str(&self.get_resource().root, "UID")
    }
    fn username(&self) -> Result<String, Errors> {
        self.get_resource()
            .get_str(&self.get_resource().root, "UNAME")
    }
}

pub trait Group: ResourceGetter {
    fn gid(&self) -> Result<i64, Errors> {
        self.get_resource()
            .get_i64(&self.get_resource().root, "GID")
    }
    fn groupname(&self) -> Result<String, Errors> {
        self.get_resource()
            .get_str(&self.get_resource().root, "GNAME")
    }
}

use xml_doc::{Document, Element};

use crate::common::errors::Errors;
use crate::common::resource::ResourceGetter;

use crate::common::permissions::{Permissions, PermissionsBits};
use crate::common::template::Template;
use crate::common::template_mut::TemplateMut;

// blanket implementation
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

pub trait GetOwner: ResourceGetter {
    fn uid(&self) -> Result<String, Errors> {
        self.get_resource()
            .get_str(&self.get_resource().root, "UID")
    }
    fn username(&self) -> Result<String, Errors> {
        self.get_resource()
            .get_str(&self.get_resource().root, "UNAME")
    }
}

pub trait GetGroup: ResourceGetter {
    fn gid(&self) -> Result<i64, Errors> {
        self.get_resource()
            .get_i64(&self.get_resource().root, "GID")
    }
    fn groupname(&self) -> Result<String, Errors> {
        self.get_resource()
            .get_str(&self.get_resource().root, "GNAME")
    }
}

pub trait GetPermissions: ResourceGetter {
    fn permissions(&self) -> Result<Permissions, Errors> {
        let document = &self.get_resource().document;
        let permissions = self
            .get_resource()
            .root
            .find(document, "PERMISSIONS")
            .unwrap();

        let uu = get_perm_field(document, permissions, "OWNER_U")?;
        let um = get_perm_field(document, permissions, "OWNER_M")?;
        let ua = get_perm_field(document, permissions, "OWNER_A")?;
        let gu = get_perm_field(document, permissions, "GROUP_U")?;
        let gm = get_perm_field(document, permissions, "GROUP_M")?;
        let ga = get_perm_field(document, permissions, "GROUP_A")?;
        let ou = get_perm_field(document, permissions, "OTHER_U")?;
        let om = get_perm_field(document, permissions, "OTHER_M")?;
        let oa = get_perm_field(document, permissions, "OTHER_A")?;

        Ok(PermissionsBits(uu, um, ua, gu, gm, ga, ou, om, oa).into())
    }
}

fn get_perm_field(document: &Document, perm_elem: Element, name: &str) -> Result<u8, Errors> {
    let perm_field = match perm_elem.find(document, name) {
        Some(e) => e,
        None => return Err(Errors::NotFound(name.to_string())),
    };

    // this pair is not expected to have a childs
    if perm_field.children(document).len() > 1 {
        Err(Errors::HasChilds(name.to_string()))
    } else {
        Ok(perm_field.text_content(document).parse::<u8>()?)
    }
}

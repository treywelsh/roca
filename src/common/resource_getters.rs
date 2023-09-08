use xml_doc::{Document, Element};

use crate::common::errors::Errors;
use crate::common::resource::{ResourceGetter, ResourceGetterMut};

use crate::common::permissions::{Permissions, PermissionsBits};
use crate::common::template::Template;
use crate::common::template_mut::TemplateMut;

// blanket implementation
impl<T> CommonGetters for T where T: ResourceGetter {}
impl<T> CommonGettersMut for T where T: ResourceGetterMut {}

// TODO: rename ?
pub trait CommonGetters: ResourceGetter {
    fn id(&self) -> Result<i64, Errors> {
        self.get_i64("ID")
    }

    fn name(&self) -> Result<String, Errors> {
        self.get_str("NAME")
    }

    fn get_str(&self, name: &str) -> Result<String, Errors> {
        let (document, element) = self.get_internal();
        let found = match element.find(document, name) {
            Some(e) => e,
            None => return Err(Errors::NotFound(name.to_string())),
        };
        if found.children(document).len() > 1 {
            Err(Errors::HasChilds(name.to_string()))
        } else {
            Ok(found.text_content(document))
        }
    }

    fn get_i64(&self, name: &str) -> Result<i64, Errors> {
        let i_str = self.get_str(name)?;

        Ok(i_str.parse::<i64>()?)
    }

    fn template(&self) -> Template {
        let (document, element) = self.get_internal();

        let template = element.find(document, "TEMPLATE").unwrap();

        Template::from_resource(document, template)
    }
}

pub trait CommonGettersMut: ResourceGetterMut {
    fn template_mut(&mut self) -> TemplateMut {
        let (document, element) = self.get_internal_mut();
        let template = element.find(document, "TEMPLATE").unwrap();

        TemplateMut::from_resource(document, template)
    }
}

pub trait GetOwner: CommonGetters {
    fn uid(&self) -> Result<String, Errors> {
        self.get_str("UID")
    }
    fn username(&self) -> Result<String, Errors> {
        self.get_str("UNAME")
    }
}

pub trait GetGroup: CommonGetters {
    fn gid(&self) -> Result<i64, Errors> {
        self.get_i64("GID")
    }
    fn groupname(&self) -> Result<String, Errors> {
        self.get_str("GNAME")
    }
}

pub trait GetPermissions: ResourceGetter {
    fn permissions(&self) -> Result<Permissions, Errors> {
        let (document, element) = self.get_internal();

        let permissions = element.find(document, "PERMISSIONS").unwrap();

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

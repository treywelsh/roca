use crate::common::errors::Errors;
use crate::common::permissions::{Permissions, PermissionsBits};

use crate::common::template::elements::Vector;

use crate::common::xml::permissions::get_field;
use crate::common::xml::resource::{XMLDocGetters, XMLDocGettersMut};
use crate::common::xml::shared_getters::BaseGetters;
use crate::common::xml::template::Template;
use crate::common::xml::template_mut::TemplateMut;

pub trait Get {
    fn get_str(&self, key: &str) -> Result<String, Errors>;
    fn get_i64(&self, key: &str) -> Result<i64, Errors>;
    fn get_vector(&self, key: &str) -> Result<Vector, Errors>;
    //fn get_vectors(&self, key: &str) -> Result<Vec<Vector>, Errors>;
}

// blanket implementation
impl<T> ResourceGettersMut for T where T: XMLDocGettersMut {}

/// Add default getters to retrieve generic resource attributes
pub trait ResourceGetters: BaseGetters {
    fn id(&self) -> Result<i64, Errors> {
        self.get_i64("ID")
    }

    fn name(&self) -> Result<String, Errors> {
        self.get_str("NAME")
    }

    fn template(&self) -> Template {
        let (document, element) = self.get_internal();

        let template = element.find(document, "TEMPLATE").unwrap();

        Template::from_resource(document, template)
    }
}
/// Add default methods to allow modifying dynamic XML content
pub trait ResourceGettersMut: XMLDocGettersMut {
    fn template_mut(&mut self) -> TemplateMut {
        let (document, element) = self.get_internal_mut();
        let template = element.find(document, "TEMPLATE").unwrap();

        TemplateMut::from_resource(document, template)
    }
}

/// Add user attributes getters
pub trait GetOwner: ResourceGetters {
    fn uid(&self) -> Result<i64, Errors> {
        self.get_i64("UID")
    }
    fn username(&self) -> Result<String, Errors> {
        self.get_str("UNAME")
    }
}

// Add group attributes getters
pub trait GetGroup: ResourceGetters {
    fn gid(&self) -> Result<i64, Errors> {
        self.get_i64("GID")
    }
    fn groupname(&self) -> Result<String, Errors> {
        self.get_str("GNAME")
    }
}

// Add permission attribute getters
pub trait GetPermissions: XMLDocGetters {
    fn permissions(&self) -> Result<Permissions, Errors> {
        let (document, element) = self.get_internal();

        let permissions = element.find(document, "PERMISSIONS").unwrap();

        let uu = get_field(document, permissions, "OWNER_U")?;
        let um = get_field(document, permissions, "OWNER_M")?;
        let ua = get_field(document, permissions, "OWNER_A")?;
        let gu = get_field(document, permissions, "GROUP_U")?;
        let gm = get_field(document, permissions, "GROUP_M")?;
        let ga = get_field(document, permissions, "GROUP_A")?;
        let ou = get_field(document, permissions, "OTHER_U")?;
        let om = get_field(document, permissions, "OTHER_M")?;
        let oa = get_field(document, permissions, "OTHER_A")?;

        Ok(PermissionsBits(uu, um, ua, gu, gm, ga, ou, om, oa).into())
    }
}

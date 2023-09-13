use crate::common::errors::Errors;
use crate::common::permissions::{Permissions, PermissionsBits};

use crate::common::xml::permissions::get_field;
use crate::common::xml::resource::XMLDocGetters;
use crate::common::xml::shared_getters::BaseGetters;
use crate::common::xml::template::Template;

/// Add default getters to retrieve generic resource attributes
pub trait ResourceGetters: BaseGetters {
    fn id(&self) -> Result<i64, Errors> {
        self.get_i64("ID")
    }

    fn name(&self) -> Result<String, Errors> {
        self.get("NAME")
    }

    fn template(&self) -> Template {
        let (document, element) = self.get_internal();

        let template = element.find(document, "TEMPLATE").unwrap();

        Template::from_resource(document, template)
    }
}

/// Add user attributes getters
pub trait GetOwner: ResourceGetters {
    fn uid(&self) -> Result<i64, Errors> {
        self.get_i64("UID")
    }
    fn username(&self) -> Result<String, Errors> {
        self.get("UNAME")
    }
}

// Add group attributes getters
pub trait GetGroup: ResourceGetters {
    fn gid(&self) -> Result<i64, Errors> {
        self.get_i64("GID")
    }
    fn groupname(&self) -> Result<String, Errors> {
        self.get("GNAME")
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

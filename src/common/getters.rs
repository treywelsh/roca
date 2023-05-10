use crate::common::errors::Errors;
use crate::common::ResourceData;
use crate::common::Template;

impl<T> ResourceInternal for T where T: ResourceData {}
impl<T> ResourcePublic for T where T: ResourceData {}

/// ResourceInternal adds default and generic getter methods for internal use (they need a full path of the attribute)
pub trait ResourceInternal: ResourceData {
    fn _get_str(&self, full_path: &str) -> Result<String, Errors> {
        self.get_data()._get_str(full_path)
    }

    fn _get_i64(&self, full_path: &str) -> Result<i64, Errors> {
        self.get_data()._get_i64(full_path)
    }

    fn _template(&self, resource_type: &str) -> Template {
        Template::new(&self.get_data().package, resource_type)
    }
}

/// ResourceInternal adds default and generic getter methods for roca user (they only need the attribute name)
pub trait ResourcePublic: ResourceData {
    fn get_str(&self, name: &str) -> Result<String, Errors> {
        self.get_data().get_str(self.get_type(), name)
    }

    fn get_i64(&self, name: &str) -> Result<i64, Errors> {
        let value = self.get_data().get_str(self.get_type(), name)?;

        let v = value.parse()?;
        Ok(v)
    }
}

#[macro_export]
macro_rules! getters {
    ($resource_type:expr) => {
        pub fn id(&self) -> Result<i64, Errors> {
            self._get_i64(concat!("/", $resource_type, "/ID"))
        }
        pub fn name(&self) -> Result<String, Errors> {
            self._get_str(concat!("/", $resource_type, "/NAME"))
        }
        pub fn template(&self) -> Template {
            self._template($resource_type)
        }
    };
}

#[macro_export]
macro_rules! user_getters {
    ($resource_type:expr) => {
        pub fn uid(&self) -> Result<String, Errors> {
            self._get_str(concat!("/", $resource_type, "/UID"))
        }
        pub fn username(&self) -> Result<String, Errors> {
            self._get_str(concat!("/", $resource_type, "/UNAME"))
        }
    };
}

#[macro_export]
macro_rules! group_getters {
    ($resource_type:expr) => {
        pub fn gid(&self) -> Result<String, Errors> {
            self._get_str(concat!("/", $resource_type, "/GID"))
        }
        pub fn groupname(&self) -> Result<String, Errors> {
            self._get_str(concat!("/", $resource_type, "/GNAME"))
        }
    };
}

//#[macro_export]
//macro_rules! template_getters {
//    ($resource_type:expr) => {
//        pub fn template(&self) -> Template {
//            self._template($resource_type)
//        }
//    };
//}

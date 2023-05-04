use crate::common::errors::Errors;
use crate::common::ResourceData;
use crate::common::Template;

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

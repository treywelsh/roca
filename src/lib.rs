//! An OpenNebula XML-RPC API binding for Rust.
//!
//! The `roca` crate provides a binding for interfacing [OpenNebula XML-RPC API].
//!
//! [OpenNebula XML-RPC API]: http://docs.opennebula.org/stable/integration/system_interfaces/api.html

pub mod client;
pub mod common;
pub mod controller;
pub mod user;
pub mod vm;

pub mod prelude {
    pub use crate::client::ClientXMLRPC;
    pub use crate::common::parameters;
    pub use crate::common::permissions::{Permissions, PermissionsBits};
    pub use crate::common::resource::{Resource, ResourceGetter};
    pub use crate::common::resource_getters::{CommonGetters, GetGroup, GetOwner, GetPermissions};
    pub use crate::common::template_builder::TemplateBuilder;
    pub use crate::common::template_getters::TemplateCommonGetters;
    pub use crate::controller::Controller;
}

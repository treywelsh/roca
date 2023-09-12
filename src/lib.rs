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
pub mod vm_pool;

pub mod prelude {
    pub use crate::client::ClientXMLRPC;
    pub use crate::common::parameters;
    pub use crate::common::resource_getters::{
        GetGroup, GetOwner, GetPermissions, ResourceGetters,
    };
    pub use crate::common::template::builder as template;
    pub use crate::common::template::elements::Vector;
    pub use crate::controller::Controller;

    pub use crate::common::xml::resource::Resource;
    pub use crate::common::xml::shared_getters::BaseGetters;
}

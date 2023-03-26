//! An OpenNebula XML-RPC API binding for Rust.
//!
//! The `roca` crate provides a binding for interfacing [OpenNebula XML-RPC API].
//!
//! [OpenNebula XML-RPC API]: http://docs.opennebula.org/stable/integration/system_interfaces/api.html

pub mod client;
pub mod common;
pub mod controller;
pub mod user;

pub mod prelude {
    pub use crate::client::ClientXMLRPC;
    pub use crate::common::getters::ResourcePublic;
    pub use crate::controller::Controller;
}

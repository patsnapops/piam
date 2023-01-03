//! Generic proxy core that work with custom plugins such as [`piam_object_storage`] crate.

#![warn(clippy::nursery)]
// #![feature(custom_inner_attributes)]
// #![clippy::cognitive_complexity = "10"]

pub mod config;
pub mod container;
pub mod error;
pub mod manager_api;
pub mod policy;
pub mod request;
pub mod response;
pub mod signature;
pub mod state;
pub mod type_alias;

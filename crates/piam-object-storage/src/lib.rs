//! IAM plugin for object storage data model that implements s3 protocol.

// #![warn(clippy::nursery)]
// #![feature(custom_inner_attributes)]
// #![clippy::cognitive_complexity = "10"]

pub mod config;
pub mod error;
pub mod input;
pub mod parser;
#[cfg(feature = "cos-parser")]
pub mod parser_cos;
pub mod parser_s3;
pub mod policy;

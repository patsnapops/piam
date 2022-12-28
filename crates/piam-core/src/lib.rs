// #![allow(unused)]

pub mod account;
pub mod condition;
pub mod config;
pub mod crypto;
pub mod effect;
pub mod group;
pub mod input;
pub mod manager_api_constant;
pub mod policy;
pub mod principal;
pub mod relation_model;
pub mod type_alias;

pub trait IamIdentity {
    fn id_str(&self) -> &str;
}

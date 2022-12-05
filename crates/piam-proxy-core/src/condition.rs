use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::type_alias::HttpRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    pub ip_addr: Option<IpAddr>,
    pub region: Option<Region>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConditionRange {
    pub ip_addr: Option<Vec<IpAddr>>,
    pub region: Option<Vec<Region>>,
}

impl Condition {}

pub trait ConditionExt {
    fn condition(&self) -> Condition;
}

impl ConditionExt for HttpRequest {
    fn condition(&self) -> Condition {
        Condition {
            ip_addr: None,
            region: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Region {
    #[default]
    Unknown,
}

use std::{net::IpAddr, time::Instant};

#[allow(unused_imports)]
use cidr::IpCidr;
use serde::{Deserialize, Serialize};

use crate::type_alias::HttpRequest;

pub struct Condition {
    pub ip_addr: Option<IpAddr>,
    pub region: Option<Region>,
    pub time: Instant,
}

pub struct Conditions {
    pub ip_addr: Vec<IpAddr>,
    pub region: Vec<Region>,
    pub times: Vec<(Instant, Instant)>,
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
            time: Instant::now(),
        }
    }
}

pub enum Region {
    Unknown,
}

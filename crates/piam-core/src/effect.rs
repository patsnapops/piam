use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "allow")]
    Allow {
        #[serde(skip_serializing_if = "Option::is_none")]
        emit_event: Option<EmitEvent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rate_limit: Option<RateLimit>,
        #[serde(skip_serializing_if = "Option::is_none")]
        modify: Option<Modify>,
    },
    #[serde(rename = "deny")]
    Deny(Option<EmitEvent>),
}

impl Effect {
    pub fn allow() -> Self {
        Self::Allow {
            emit_event: None,
            rate_limit: None,
            modify: None
        }
    }

    pub fn deny() -> Self {
        Self::default()
    }
}

impl Default for Effect {
    fn default() -> Self {
        Self::Deny(None)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct EmitEvent {
    pub log: Option<Log>,
    pub metric: Option<Metric>,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub address: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub address: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct RateLimit {
    pub duration: Duration,
    pub count: u32,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Modify {}

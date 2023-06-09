use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    /// If multiple effects hit with all Allow, the request should be allowed
    #[serde(rename = "allow")]
    Allow {
        /// If multiple effects hit, all of them can emit event
        #[serde(skip_serializing_if = "Option::is_none")]
        emit_event: Option<EmitEvent>,
        /// If multiple effects hit, there can only be one rate_limit
        #[serde(skip_serializing_if = "Option::is_none")]
        rate_limit: Option<RateLimit>,
        /// If multiple effects hit, there can only be one modify
        #[serde(skip_serializing_if = "Option::is_none")]
        modify: Option<Modify>,
    },
    /// If multiple effects hit with both Allow and Deny, the request should be denied
    #[serde(rename = "deny")]
    Deny(Option<EmitEvent>),
}

impl Effect {
    pub fn allow() -> Self {
        Self::Allow {
            emit_event: None,
            rate_limit: None,
            modify: None,
        }
    }

    pub fn deny() -> Self {
        Self::default()
    }

    pub fn is_allow(&self) -> bool {
        matches!(self, Self::Allow { .. })
    }

    pub fn is_deny(&self) -> bool {
        matches!(self, Self::Deny(_))
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

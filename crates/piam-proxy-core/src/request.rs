use std::fmt::Debug;

use thiserror::Error;

use crate::{
    effect::Effect,
    response,
    type_alias::{ApplyResult, HttpRequest},
};

#[derive(Error, Debug)]
pub enum ParserError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    UnknownOperation(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown parser error")]
    Unknown,
}

pub trait HttpRequestExt {
    fn apply_effect(self, effect: &Effect) -> ApplyResult;
}

impl HttpRequestExt for HttpRequest {
    fn apply_effect(self, effect: &Effect) -> ApplyResult {
        match effect {
            Effect::Allow { .. } => {
                // TODO: impl Allow stuff
                ApplyResult::Forward(self)
            }
            Effect::Deny(_maybe_emit) => {
                // TODO: impl Deny stuff
                ApplyResult::Reject(response::rejected_by_policy())
            }
        }
    }
}

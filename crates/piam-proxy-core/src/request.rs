use std::fmt::Debug;

use anyhow::Result;
use log::debug;
use thiserror::Error;

use crate::{
    condition::ConditionExt,
    effect::Effect,
    error::ProxyResult,
    input::Input,
    policy::{PolicyContainer, Statement},
    principal::{Group, PrincipalContainer, User},
    response,
    sign::AmzExt,
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

pub trait Parser<I: Input> {
    fn parse(&self, req: &HttpRequest) -> Result<I>;
}

pub trait HttpRequestExt {
    fn apply_policies<S, I>(
        self,
        principal_container: &PrincipalContainer,
        policy_container: &PolicyContainer<S>,
    ) -> ApplyResult
    where
        S: Statement<Input = I> + Debug,
        I: Input;

    fn apply_effect(self, maybe_effect: Option<&Effect>) -> ApplyResult;
}

impl HttpRequestExt for HttpRequest {
    fn apply_policies<S, I>(
        self,
        principal_container: &PrincipalContainer,
        policy_container: &PolicyContainer<S>,
    ) -> ApplyResult
    where
        S: Statement<Input = I> + Debug,
        I: Input,
    {
        let ak = match self.extract_access_key() {
            Err(e) => return ApplyResult::Reject(response::forbidden(&format!("{:?}", e))),
            Ok(ak) => ak,
        };
        let user = match principal_container.find_user_by_access_key(ak) {
            None => return ApplyResult::Reject(response::user_not_found()),
            Some(u) => u,
        };
        debug!("{:#?}", user);
        let group = match principal_container.find_group_by_user(user) {
            None => return ApplyResult::Reject(response::group_not_found()),
            Some(g) => g,
        };
        debug!("{:#?}", group);
        let policies = match policy_container.find_policies_by_group(group) {
            None => return ApplyResult::Reject(response::policy_not_found()),
            Some(p) => p,
        };

        let input = Input::from_http(&self).expect("parse input error");

        let maybe_effect = policies.iter().find_map(|policy| {
            debug!("{:#?}", policy);
            // TODO: find condition
            let _condition = self.condition();
            // let _condition_policy = &policy.conditions;

            policy.statement.find_effect_for_input(&input)
        });

        self.apply_effect(maybe_effect)
    }

    fn apply_effect(self, maybe_effect: Option<&Effect>) -> ApplyResult {
        match maybe_effect {
            Some(effect) => match effect {
                Effect::Allow { .. } => {
                    // TODO: impl Allow stuff
                    ApplyResult::Forward(self)
                }
                Effect::Deny(_maybe_emit) => {
                    // TODO: impl Deny stuff
                    ApplyResult::Reject(response::rejected_by_policy())
                }
            },
            None => ApplyResult::Reject(response::effect_not_found()),
        }
    }
}

pub trait ParserExt<I: Input> {
    fn parpar(&self) -> Result<I>;
}

use std::fmt::Debug;

use log::debug;
use thiserror::Error;

use crate::{
    effect::Effect,
    error::{ProxyError, ProxyResult},
    input::Input,
    policy::{Policies, PolicyContainer, Statement},
    principal::PrincipalContainer,
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

pub fn find_policies_by_access_key<'a, S, I>(
    access_key: &str,
    principal_container: &PrincipalContainer,
    policy_container: &'a PolicyContainer<S>,
) -> ProxyResult<&'a Policies<S>>
where
    S: Statement<Input = I> + Debug,
    I: Input,
{
    debug!("{:#?}", principal_container);
    let user = principal_container
        .find_user_by_access_key(access_key)
        .ok_or_else(|| {
            ProxyError::UserNotFound(format!("User not found for access key: {}", access_key))
        })?;
    debug!("{:#?}", user);
    let group = principal_container
        .find_group_by_user(user)
        .ok_or_else(|| {
            ProxyError::GroupNotFound(format!("Group not found for user: {}", user.id))
        })?;
    debug!("{:#?}", group);
    let policies = policy_container
        .find_policies_by_group(group)
        .ok_or_else(|| {
            ProxyError::PolicyNotFound(format!("Policy not found for group: {}", group.id))
        })?;
    Ok(policies)
}

pub fn find_effect<S, I>(policies: &Policies<S>, input: I) -> ProxyResult<&Effect>
where
    S: Statement<Input = I> + Debug,
    I: Input,
{
    policies
        .iter()
        .find_map(|policy| {
            debug!("{:#?}", policy);
            // TODO: find condition
            // let _condition = self.condition();
            // let _condition_policy = &policy.conditions;
            policy.statement.find_effect_for_input(&input)
        })
        .ok_or_else(|| {
            ProxyError::EffectNotFound(format!("Effect not found for input: {:?}", input))
        })
}

use std::fmt::Debug;

use log::debug;
use thiserror::Error;

use crate::{
    config,
    effect::Effect,
    error::{ProxyError, ProxyResult},
    response,
    type_alias::{HttpClient, HttpRequest, HttpResponse},
};

pub enum ApplyResult {
    Forward(HttpRequest),
    Reject(HttpResponse),
}

pub trait HttpRequestExt {
    fn apply_effects(self, effect: Vec<&Effect>) -> ProxyResult<ApplyResult>;
}

impl HttpRequestExt for HttpRequest {
    fn apply_effects(self, effects: Vec<&Effect>) -> ProxyResult<ApplyResult> {
        if effects.is_empty() {
            return Err(ProxyError::EffectNotFound(format!(
                "Access Denied: No matching policy, user request: {:?}",
                self.into_parts().0
            )));
        };

        // TODO: try apply multiple effects
        if effects.len() > 1 {
            return Err(ProxyError::OtherInternal(
                "Multiple effects found when apply_effects, not supported yet".into(),
            ));
        };

        Ok(match effects.first().expect("todo") {
            Effect::Allow { .. } => {
                // TODO: impl Allow stuff
                ApplyResult::Forward(self)
            }
            Effect::Deny(_maybe_emit) => {
                // TODO: impl Deny stuff
                ApplyResult::Reject(response::rejected_by_policy())
            }
        })
    }
}

pub fn from_region_to_host(region: &str) -> ProxyResult<&'static str> {
    match region {
        "cn-northwest-1" => Ok("s3.cn-northwest-1.amazonaws.com.cn"),
        "us-east-1" => Ok("s3.us-east-1.amazonaws.com"),
        "eu-central-1" => Ok("s3.eu-central-1.amazonaws.com"),
        _ => Err(ProxyError::InvalidRegion(format!(
            "unsupported region: {}",
            region,
        ))),
    }
}

pub async fn forward(new_req: HttpRequest, client: &HttpClient) -> ProxyResult<HttpResponse> {
    debug!("new_req headers {:#?}", new_req.headers());
    let res = client
        .request(new_req)
        .await
        .expect("request to s3 service should not fail");
    Ok(res)
}

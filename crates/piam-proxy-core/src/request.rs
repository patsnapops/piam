use log::debug;

use crate::{
    effect::Effect,
    error::{esome_context, ProxyError, ProxyResult},
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

        Ok(match esome_context(effects.first(), "apply_effects todo") {
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

pub fn from_region_to_endpoint(region: &str) -> ProxyResult<String> {
    Ok(format!("http://{}", from_region_to_host(region)?))
}

pub fn from_region_to_host(region: &str) -> ProxyResult<&'static str> {
    match region {
        // aws
        "cn-northwest-1" => Ok("s3.cn-northwest-1.amazonaws.com.cn"),
        "us-east-1" => Ok("s3.us-east-1.amazonaws.com"),
        "eu-central-1" => Ok("s3.eu-central-1.amazonaws.com"),
        // tencent
        "ap-shanghai" => Ok("cos.ap-shanghai.myqcloud.com"),
        "na-ashburn" => Ok("cos.na-ashburn.myqcloud.com"),
        _ => Err(ProxyError::InvalidRegion(format!(
            "unsupported region: {}",
            region,
        ))),
    }
}

pub async fn forward(new_req: HttpRequest, client: &HttpClient) -> ProxyResult<HttpResponse> {
    debug!("new_req headers {:#?}", new_req.headers());
    let res = client.request(new_req).await;
    res.map_err(|e| ProxyError::OtherInternal(format!("proxy forwarding error: {}", e)))
}

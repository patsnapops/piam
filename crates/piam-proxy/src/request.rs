use log::debug;
use piam_core::effect::Effect;

use crate::{
    error::{ProxyError, ProxyResult},
    type_alias::{HttpClient, HttpRequest, HttpResponse},
};

pub trait HttpRequestExt {
    fn apply_effects(self, effect: Vec<&Effect>) -> ProxyResult<HttpRequest>;
}

impl HttpRequestExt for HttpRequest {
    fn apply_effects(self, effects: Vec<&Effect>) -> ProxyResult<HttpRequest> {
        if effects.is_empty() {
            return Err(ProxyError::EffectNotFound(format!(
                "access denied: no matching effects found for policy, user request: {:?}",
                self.into_parts().0
            )));
        };

        let mut allow_effects = Vec::new();
        let mut deny_effects = Vec::new();
        for effect in effects {
            match effect {
                Effect::Allow { .. } => allow_effects.push(effect),
                Effect::Deny(_) => deny_effects.push(effect),
            }
        }

        if !deny_effects.is_empty() {
            // TODO: impl Deny stuff
            return Err(ProxyError::EffectNotFound(format!(
                "access denied: by deny effect found in policy, user request: {:?}",
                self.into_parts().0
            )));
        }

        if allow_effects.is_empty() {
            Err(ProxyError::EffectNotFound(format!(
                "access denied: no matching allow effects found for policy, user request: {:?}",
                self.into_parts().0
            )))
        } else {
            // TODO: impl Allow stuff
            Ok(self)
        }
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
            "unsupported region: {region}",
        ))),
    }
}

pub async fn forward(new_req: HttpRequest, client: &HttpClient) -> ProxyResult<HttpResponse> {
    debug!("new_req headers {:#?}", new_req.headers());
    let res = client.request(new_req).await;
    res.map_err(|e| ProxyError::OtherInternal(format!("proxy forwarding error: {e}")))
}

use http::{Response, StatusCode};
use hyper::Body;

use crate::type_alias::HttpResponse;

pub fn policy_not_found() -> HttpResponse {
    forbidden("PolicyNotFound")
}

pub fn rejected_by_policy() -> HttpResponse {
    forbidden("RejectedByPolicy")
}

pub fn effect_not_found() -> HttpResponse {
    forbidden("EffectNotFound")
}

pub fn forbidden(body: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::from(body.to_string()))
        .unwrap()
}

use http::{Response, StatusCode};
use http::header::CONTENT_TYPE;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

use crate::type_alias::HttpResponse;

pub fn extract_access_key_error(message: &str) -> HttpResponse {
    forbidden("ExtractAccessKeyError", message)
}

pub fn user_not_found() -> HttpResponse {
    forbidden("UserNotFound", "Check Users")
}

pub fn group_not_found() -> HttpResponse {
    forbidden("GroupNotFound", "Check Groups")
}

pub fn policy_not_found() -> HttpResponse {
    forbidden("PolicyNotFound", "Check Policies")
}

pub fn rejected_by_policy() -> HttpResponse {
    forbidden("RejectedByPolicy", "RejectedByPolicy")
}

pub fn effect_not_found() -> HttpResponse {
    forbidden("EffectNotFound", "Effect not found, rejected by default policy")
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Error")]
#[serde(rename_all = "PascalCase")]
pub struct AwsErrorXml {
    pub code: String,
    pub message: String,
    #[serde(rename = "AWSAccessKeyId")]
    pub aws_access_key_id: String,
    pub host_id: String,
}

pub fn bad_request(code: &str, message: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message)))
        .expect("build bad_request response should not fail")
}

pub fn forbidden(code: &str, message: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message)))
        .expect("build forbidden response should not fail")
}

fn error_payload(code: &str, message: &str) -> String {
    #[cfg(feature = "aws-xml-response")]
    aws_xml_error_payload(code, message)
}

fn aws_xml_error_payload(code: &str, message: &str) -> String {
    let error = AwsErrorXml {
        code: format!("Piam{}", code),
        message: format!("[PIAM] {}", message),
        aws_access_key_id: "".into(),
        host_id: "".into()
    };
    serde_xml_rs::to_string(&error).expect("ser_xml should not fail")
}

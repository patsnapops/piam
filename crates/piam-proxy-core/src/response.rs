use axum::response::IntoResponse;
use http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode};
use hyper::Body;
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::PROXY_TYPE, error::ProxyError, type_alias::HttpResponse};

pub trait HttpResponseExt {
    fn add_piam_headers(self, id: String) -> Self;

    fn add_piam_headers_with_random_id(self) -> Self;
}

impl HttpResponseExt for HttpResponse {
    fn add_piam_headers(mut self, id: String) -> Self {
        let headers = self.headers_mut();
        headers.append(
            "x-patsnap-proxy-type",
            HeaderValue::from_static("Patsnap S3 Proxy"),
        );
        headers.append("x-patsnap-request-id", HeaderValue::from_str(&id).unwrap());
        self
    }

    fn add_piam_headers_with_random_id(self) -> Self {
        self.add_piam_headers(Uuid::new_v4().to_string())
    }
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> axum::response::Response {
        let id = Uuid::new_v4().to_string();
        let proxy = PROXY_TYPE.load();
        let trace_info = format!("Proxy: {} x-patsnap-request-id: {}", proxy, id);
        let (res, err_type) = match &self {
            ProxyError::BadRequest(msg)
            | ProxyError::OperationNotSupported(msg)
            | ProxyError::InvalidRegion(msg) => (
                bad_request("BadRequest", &format!("{} {}", msg, trace_info)),
                "BadRequest",
            ),
            ProxyError::InvalidAuthorizationHeader(msg)
            | ProxyError::InvalidAccessKey(msg)
            | ProxyError::MissingPolicy(msg)
            | ProxyError::EffectNotFound(msg) => (
                forbidden("Forbidden", &format!("{} {}", msg, trace_info)),
                "Forbidden",
            ),
            ProxyError::OtherInternal(msg)
            | ProxyError::ManagerApi(msg)
            | ProxyError::Deserialize(msg)
            | ProxyError::UserNotFound(msg)
            | ProxyError::GroupNotFound(msg) => (
                internal_error("InternalError", &format!("{} {}", msg, trace_info)),
                "Internal",
            ),
        };
        error!(
            "x-patsnap-request-id: {} ErrorType: {} Error: {:?}",
            id, err_type, &self
        );
        res.add_piam_headers(id).into_response()
    }
}

pub fn rejected_by_policy() -> HttpResponse {
    forbidden("RejectedByPolicy", "RejectedByPolicy")
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

pub fn internal_error(code: &str, message: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message)))
        .expect("build internal error response should not fail")
}

fn error_payload(code: &str, message: &str) -> String {
    #[cfg(feature = "aws-xml-response")]
    aws_xml_error_payload(code, message)
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

fn aws_xml_error_payload(code: &str, message: &str) -> String {
    let error = AwsErrorXml {
        code: format!("Piam{}", code),
        message: format!("[Patsnap IAM] {}", message),
        aws_access_key_id: "".into(),
        host_id: "".into(),
    };
    serde_xml_rs::to_string(&error).expect("ser_xml should not fail")
}

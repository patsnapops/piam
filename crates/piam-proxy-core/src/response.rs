use axum::response::IntoResponse;
use http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode};
use hyper::Body;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::{CLUSTER_ENV, PROXY_TYPE},
    error::ProxyError,
    type_alias::HttpResponse,
};

pub trait HttpResponseExt {
    fn add_piam_headers(self, id: String) -> Self;

    fn add_piam_headers_with_random_id(self) -> Self;
}

impl HttpResponseExt for HttpResponse {
    fn add_piam_headers(mut self, id: String) -> Self {
        let headers = self.headers_mut();
        headers.append(
            "x-patsnap-proxy-type",
            HeaderValue::from_static(&PROXY_TYPE.load()),
        );
        headers.append(
            "x-patsnap-proxy-cluster-env",
            HeaderValue::from_str(&CLUSTER_ENV.load()).expect("CLUSTER_ENV should be valid"),
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
        let r_t = |resp_fn: fn(&str, &str, &str) -> HttpResponse, msg, err_type| {
            let trace_info = format!(
                "proxy_type: {}, proxy_cluster_env: {}, \
                error_type: {}, message: {}, x-patsnap-request-id: {}",
                PROXY_TYPE.load(),
                CLUSTER_ENV.load(),
                err_type,
                msg,
                id
            );
            (resp_fn(err_type, &trace_info, &id), trace_info)
        };
        let res = match &self {
            ProxyError::BadRequest(msg)
            | ProxyError::InvalidEndpoint(msg)
            | ProxyError::InvalidRegion(msg)
            | ProxyError::InvalidAuthorizationHeader(msg) => {
                let (r, t) = r_t(bad_request, msg, self.name());
                info!("{}", t);
                r
            }
            ProxyError::InvalidAccessKey(msg)
            | ProxyError::OperationNotSupported(msg)
            | ProxyError::MissingPolicy(msg)
            | ProxyError::EffectNotFound(msg) => {
                let (r, t) = r_t(forbidden, msg, self.name());
                warn!("{}", t);
                r
            }
            ProxyError::OtherInternal(msg)
            | ProxyError::ManagerApi(msg)
            | ProxyError::Deserialize(msg)
            | ProxyError::UserNotFound(msg)
            | ProxyError::GroupNotFound(msg) => {
                let (r, t) = r_t(internal_err, msg, self.name());
                error!("{}", t);
                r
            }
        };
        res.add_piam_headers(id).into_response()
    }
}

pub fn rejected_by_policy() -> HttpResponse {
    forbidden(
        "RejectedByPolicy",
        "RejectedByPolicy",
        &Uuid::new_v4().to_string(),
    )
}

pub fn bad_request(code: &str, message: &str, request_id: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message, request_id)))
        .expect("build bad_request response should not fail")
}

pub fn forbidden(code: &str, message: &str, request_id: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message, request_id)))
        .expect("build forbidden response should not fail")
}

pub fn internal_err(code: &str, message: &str, request_id: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message, request_id)))
        .expect("build internal error response should not fail")
}

fn error_payload(code: &str, message: &str, request_id: &str) -> String {
    #[cfg(feature = "aws-xml-response")]
    aws_xml_error_payload(code, message, request_id)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Error")]
#[serde(rename_all = "PascalCase")]
pub struct AwsErrorXml {
    pub code: String,
    pub message: String,
    #[serde(rename = "AWSAccessKeyId")]
    pub aws_access_key: String,
    pub request_id: String,
    pub host_id: String,
}

fn aws_xml_error_payload(code: &str, message: &str, request_id: &str) -> String {
    let error = AwsErrorXml {
        code: format!("Piam{}", code),
        message: format!("PIAM {}", message),
        aws_access_key: "".into(),
        request_id: request_id.into(),
        host_id: "".into(),
    };
    serde_xml_rs::to_string(&error).expect("ser_xml should not fail")
}

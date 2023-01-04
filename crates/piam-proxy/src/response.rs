use axum::response::IntoResponse;
use busylib::prelude::{EnhancedExpect, EnhancedUnwrap};
use http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode};
use hyper::Body;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::{proxy_region_env, PROXY_TYPE},
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
            "x-patsnap-proxy-region-env",
            HeaderValue::from_str(&proxy_region_env()).unwp(),
        );
        headers.append("x-patsnap-request-id", HeaderValue::from_str(&id).unwp());
        self
    }

    fn add_piam_headers_with_random_id(self) -> Self {
        self.add_piam_headers(Uuid::new_v4().to_string())
    }
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> axum::response::Response {
        let id = Uuid::new_v4().to_string();
        let response_and_trace = |resp_fn: fn(&str, &str, &str) -> HttpResponse, msg, err_type| {
            let trace_info = format!(
                "proxy_type: {}, proxy_region_env: {}, \
                error_type: {}, message: {}, x-patsnap-request-id: {}",
                PROXY_TYPE.load(),
                proxy_region_env(),
                err_type,
                msg,
                id
            );
            (resp_fn(err_type, &trace_info, &id), trace_info)
        };
        let res = match &self {
            Self::BadRequest(msg)
            | Self::MalformedProtocol(msg)
            | Self::ResourceNotFound(msg)
            | Self::InvalidEndpoint(msg)
            | Self::InvalidRegion(msg)
            | Self::InvalidAuthorizationHeader(msg) => {
                let (r, t) = response_and_trace(bad_request, msg, self.name());
                info!("{}", t);
                r
            }
            Self::InvalidAccessKey(msg)
            | Self::ParserError(msg)
            | Self::OperationNotSupported(msg)
            | Self::GroupNotFound(msg)
            | Self::MissingPolicy(msg)
            | Self::EffectNotFound(msg) => {
                let (r, t) = response_and_trace(forbidden, msg, self.name());
                dbg!(&t);
                warn!("{}", t);
                r
            }
            Self::OtherInternal(msg)
            | Self::ManagerApi(msg)
            | Self::Deserialize(msg)
            | Self::UserNotFound(msg) => {
                let (r, t) = response_and_trace(internal_err, msg, self.name());
                error!("{}", t);
                r
            }
            Self::FatalError(msg) => {
                error!("fatal error happened: {}", msg);
                panic!("fatal error happened: {msg}");
            }
            Self::AssertFail(msg) => {
                error!("assertion failed: {}", msg);
                panic!("assertion failed: {msg}");
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
        .unwp()
}

pub fn forbidden(code: &str, message: &str, request_id: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message, request_id)))
        .unwp()
}

pub fn internal_err(code: &str, message: &str, request_id: &str) -> HttpResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, "application/xml")
        .body(Body::from(error_payload(code, message, request_id)))
        .unwp()
}

fn error_payload(code: &str, message: &str, request_id: &str) -> String {
    #[cfg(feature = "aws-xml-response")]
    return aws_xml_error_payload(code, message, request_id);
    #[cfg(not(feature = "aws-xml-response"))]
    "error_payload must have default impl".into()
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

#[cfg(feature = "aws-xml-response")]
fn aws_xml_error_payload(code: &str, message: &str, request_id: &str) -> String {
    let error = AwsErrorXml {
        code: format!("Piam{}", code),
        message: format!("PIAM {}", message),
        aws_access_key: "".into(),
        request_id: request_id.into(),
        host_id: "".into(),
    };
    serde_xml_rs::to_string(&error).ex("aws_xml_error_payload should not fail")
}

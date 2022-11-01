use std::time::SystemTime;

use anyhow::Result;
use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};
use http::{header::AUTHORIZATION, Request};
use hyper::{body, Body};

use crate::{
    amz_canonical_request::header::{X_AMZ_CONTENT_SHA_256, X_AMZ_DATE, X_AMZ_SECURITY_TOKEN},
    config::CORE_CONFIG,
    error::{ProxyError, ProxyResult},
    type_alias::HttpRequest,
};

pub trait AmzExt {
    fn extract_access_key(&self) -> ProxyResult<&str>;
}

impl AmzExt for HttpRequest {
    fn extract_access_key(&self) -> ProxyResult<&str> {
        let auth = self.headers().get(AUTHORIZATION).ok_or_else(|| {
            ProxyError::InvalidAuthorizationHeader("Missing authorization header".into())
        })?;
        let auth_str = auth.to_str().map_err(|_| {
            ProxyError::InvalidAuthorizationHeader(format!(
                "Malformed authorization header, only visible ASCII chars allowed. \
                The authorization header: {:#?}",
                auth
            ))
        })?;
        // example auth_str: "AWS4-HMAC-SHA256 Credential=AKPSSVCSPROXYDEV/20221012/cn-northwest-1/s3/aws4_request ..."
        let credential_and_ak = auth_str
            .split('/')
            .next()
            .and_then(|s| s.split_once('='))
            .ok_or_else(|| {
                ProxyError::InvalidAuthorizationHeader(format!(
                    "Malformed authorization header\
                    (not a valid AMZ sigV4 authorization header): {}",
                    auth_str
                ))
            })?;
        Ok(credential_and_ak.1)
    }
}

pub async fn sign_with_amz_params(mut req: HttpRequest) -> Result<HttpRequest> {
    // save checksum before signing
    let checksum = req
        .headers()
        .get(X_AMZ_CONTENT_SHA_256)
        .expect("X_AMZ_CONTENT_SHA_256 not found while sign_with_amz_params")
        .clone();

    // see `aws_sigv4::http_request::sign::calculate_signing_headers`
    req.headers_mut().remove(X_AMZ_DATE);
    req.headers_mut().remove(X_AMZ_CONTENT_SHA_256);
    req.headers_mut().remove(X_AMZ_SECURITY_TOKEN);
    req.headers_mut().remove(AUTHORIZATION);
    // x-forwarded-for causes SignatureDoesNotMatch(from aws),
    // it is usually added by gateways like kong, which used by patsnap
    req.headers_mut().remove("x-forwarded-for");

    // convert body to bytes for signing
    let (p, b) = req.into_parts();
    let bytes = body::to_bytes(b).await?;
    let mut byte_req = Request::from_parts(p, bytes);

    // do signing
    let signing_settings = SigningSettings::default();
    let params = &CORE_CONFIG.load().amz_sign_params;
    let signing_params = SigningParams::builder()
        .access_key(&params.access_key)
        .secret_key(&params.secret_key)
        .region(&params.region)
        .service_name(&params.service)
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()?;
    let signable_request = SignableRequest::from(&byte_req);
    let (signing_instructions, _signature) = sign(signable_request, &signing_params)
        .expect("Failed to sign request with AMZ sigV4")
        .into_parts();
    signing_instructions.apply_to_request(&mut byte_req);

    // restore body from bytes
    let (p, b) = byte_req.into_parts();
    req = Request::from_parts(p, Body::from(b));

    // restore checksum after signing
    req.headers_mut().insert(X_AMZ_CONTENT_SHA_256, checksum);
    Ok(req)
}

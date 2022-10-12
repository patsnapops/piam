use std::time::SystemTime;

use anyhow::Result;
use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};
use http::{header::AUTHORIZATION, HeaderValue, Request};
use hyper::{body, Body};

use crate::{
    amz_canonical_request::header::{X_AMZ_CONTENT_SHA_256, X_AMZ_DATE, X_AMZ_SECURITY_TOKEN},
    config::CORE_CONFIG,
    type_alias::HttpRequest,
};
use crate::error::{ProxyError, ProxyResult};

pub trait AmzExt {
    fn extract_access_key(&self) -> ProxyResult<&str>;
}

impl AmzExt for HttpRequest {
    fn extract_access_key(&self) -> ProxyResult<&str> {
        let auth = match self.headers().get(AUTHORIZATION) {
            None => Err(ProxyError::InvalidRequest("missing authorization header".into())),
            Some(auth) => Ok(auth)
        }?;
        let split = auth.to_str().unwrap();
        let split = split.split('/').next().unwrap();
        let split = split.split_once('=').unwrap().1;
        Ok(split)
    }
}

pub async fn sign_with_amz_params(mut req: HttpRequest) -> Result<HttpRequest> {
    // save checksum before signing
    let checksum = req.headers().get(X_AMZ_CONTENT_SHA_256).unwrap().clone();

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
        .unwrap()
        .into_parts();
    signing_instructions.apply_to_request(&mut byte_req);

    // restore body from bytes
    let (p, b) = byte_req.into_parts();
    req = Request::from_parts(p, Body::from(b));

    // restore checksum after signing
    req.headers_mut().insert(X_AMZ_CONTENT_SHA_256, checksum);
    Ok(req)
}

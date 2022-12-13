use crate::error::ProxyResult;

pub mod aws {
    use std::time::SystemTime;

    use anyhow::Result;
    use async_trait::async_trait;
    use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};
    use http::{header::AUTHORIZATION, Request};
    use hyper::{body, Body};

    use crate::{
        account::aws::AwsAccount,
        error::{ProxyError, ProxyResult},
        signature::aws::canonical_request::header::{
            X_AMZ_CONTENT_SHA_256, X_AMZ_DATE, X_AMZ_SECURITY_TOKEN,
        },
        type_alias::HttpRequest,
    };

    mod canonical_request {
        #![allow(unused)]

        /*
         * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
         * SPDX-License-Identifier: Apache-2.0
         */
        use std::{convert::TryFrom, str::FromStr};

        pub mod header {
            pub const X_AMZ_CONTENT_SHA_256: &str = "x-amz-content-sha256";
            pub const X_AMZ_DATE: &str = "x-amz-date";
            pub const X_AMZ_SECURITY_TOKEN: &str = "x-amz-security-token";
            pub const X_AMZ_USER_AGENT: &str = "x-amz-user-agent";
        }

        pub mod param {
            pub const X_AMZ_ALGORITHM: &str = "X-Amz-Algorithm";
            pub const X_AMZ_CREDENTIAL: &str = "X-Amz-Credential";
            pub const X_AMZ_DATE: &str = "X-Amz-Date";
            pub const X_AMZ_EXPIRES: &str = "X-Amz-Expires";
            pub const X_AMZ_SECURITY_TOKEN: &str = "X-Amz-Security-Token";
            pub const X_AMZ_SIGNED_HEADERS: &str = "X-Amz-SignedHeaders";
            pub const X_AMZ_SIGNATURE: &str = "X-Amz-Signature";
        }

        pub const HMAC_256: &str = "AWS4-HMAC-SHA256";

        const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";
        const STREAMING_UNSIGNED_PAYLOAD_TRAILER: &str = "STREAMING-UNSIGNED-PAYLOAD-TRAILER";
    }

    #[async_trait]
    pub trait AwsSigv4 {
        fn extract_aws_access_key_and_region(&self) -> ProxyResult<(&str, &str)>;

        async fn sign_with_aws_sigv4_params(
            self,
            params: &AwsSigv4SignParams<'_>,
        ) -> Result<HttpRequest>;
    }

    #[async_trait]
    impl AwsSigv4 for HttpRequest {
        fn extract_aws_access_key_and_region(&self) -> ProxyResult<(&str, &str)> {
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
            extract_aws_access_key_and_region_from_auth_header(auth_str)
        }

        async fn sign_with_aws_sigv4_params(
            mut self,
            params: &AwsSigv4SignParams<'_>,
        ) -> Result<HttpRequest> {
            // save checksum before signing
            let checksum = self
                .headers()
                .get(X_AMZ_CONTENT_SHA_256)
                .expect("X_AMZ_CONTENT_SHA_256 not found while sign_with_amz_params")
                .clone();

            // see `aws_sigv4::http_request::sign::calculate_signing_headers`
            self.headers_mut().remove(X_AMZ_DATE);
            self.headers_mut().remove(X_AMZ_CONTENT_SHA_256);
            self.headers_mut().remove(X_AMZ_SECURITY_TOKEN);
            self.headers_mut().remove(AUTHORIZATION);
            // x-forwarded-for causes SignatureDoesNotMatch(from aws),
            // it is usually added by gateways like kong, which used by patsnap
            self.headers_mut().remove("x-forwarded-for");

            // convert body to bytes for signing
            let (p, b) = self.into_parts();
            let bytes = body::to_bytes(b).await?;
            let mut byte_req = Request::from_parts(p, bytes);

            // do signing
            let signing_settings = SigningSettings::default();
            let signing_params = SigningParams::builder()
                .access_key(&params.account.access_key)
                .secret_key(&params.account.secret_key)
                .region(params.region)
                .service_name(params.service)
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
            self = Request::from_parts(p, Body::from(b));

            // restore checksum after signing
            self.headers_mut().insert(X_AMZ_CONTENT_SHA_256, checksum);
            Ok(self)
        }
    }

    /// example auth_str: "AWS4-HMAC-SHA256 Credential=AKPSSVCSPROXYDEV/20221012/cn-northwest-1/s3/aws4_request ..."
    fn extract_aws_access_key_and_region_from_auth_header(
        auth_str: &str,
    ) -> ProxyResult<(&str, &str)> {
        let mut split = auth_str.split('/');
        let access_key = split
            .next()
            .and_then(|s| s.split_once('='))
            .ok_or_else(|| {
                ProxyError::InvalidAuthorizationHeader(format!(
                    "Malformed authorization header found when extract access_key\
                    (not a valid AMZ sigV4 authorization header): {}",
                    auth_str
                ))
            })?
            .1;
        let region = split.nth(1).ok_or_else(|| {
            ProxyError::InvalidAuthorizationHeader(format!(
                "Malformed authorization header found when extract region\
                (not a valid AMZ sigV4 authorization header): {}",
                auth_str
            ))
        })?;
        Ok((access_key, region))
    }

    #[derive(Debug)]
    pub struct AwsSigv4SignParams<'a> {
        pub account: &'a AwsAccount,
        pub service: &'a str,
        pub region: &'a str,
    }

    #[cfg(test)]
    mod test {
        use crate::{
            config::CN_NORTHWEST_1,
            signature::aws::extract_aws_access_key_and_region_from_auth_header,
        };

        #[test]
        fn test_extract_aws_access_key_and_region_from_auth_header() {
            let (key, region) = extract_aws_access_key_and_region_from_auth_header(
                "AWS4-HMAC-SHA256 Credential=AKPSSVCSPROXYDEV/20221012/cn-northwest-1/s3/aws4_request ..."
            ).unwrap();
            assert_eq!(key, "AKPSSVCSPROXYDEV");
            assert_eq!(region, CN_NORTHWEST_1);
        }
    }
}

#[allow(unused)]
pub fn split_to_base_and_account_code(access_key: &str) -> ProxyResult<(&str, &str)> {
    // TODO: split_to_base_and_account_code
    todo!("later")
}

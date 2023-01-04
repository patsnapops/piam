use piam_core::type_alias::HttpRequest;

use crate::{
    error::ProxyResult,
    signature::{aws::AwsSigv4, tencent::TencentSig},
};

pub trait Extract {
    fn extract_access_key_and_region(&self) -> ProxyResult<(&str, &str)>;
}

impl Extract for HttpRequest {
    fn extract_access_key_and_region(&self) -> ProxyResult<(&str, &str)> {
        #[cfg(feature = "tencent-signature")]
        {
            let from_tencent_sdk = match self.headers().get(http::header::AUTHORIZATION) {
                None => false,
                Some(hv) => hv
                    .to_str()
                    .map_err(|_| {
                        crate::error::ProxyError::MalformedProtocol(
                            "authorization must only contains visible ASCII chars".to_string(),
                        )
                    })?
                    .starts_with("q-sign-algorithm"),
            };
            if from_tencent_sdk {
                return self.extract_access_key_and_region_from_tencent();
            }
        }
        self.extract_access_key_and_region_from_aws()
    }
}

pub mod aws {
    use std::time::SystemTime;

    use async_trait::async_trait;
    use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};
    use busylib::prelude::EnhancedExpect;
    use http::{header::AUTHORIZATION, Request};
    use hyper::{body, Body};
    use piam_core::account::aws::AwsAccount;

    use crate::{
        error::{ProxyError, ProxyResult},
        signature::aws::canonical_request::header::*,
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
        fn extract_access_key_and_region_from_aws(&self) -> ProxyResult<(&str, &str)>;

        async fn sign_with_aws_sigv4_params(
            self,
            params: &AwsSigv4SignParams<'_>,
        ) -> ProxyResult<HttpRequest>;
    }

    #[async_trait]
    impl AwsSigv4 for HttpRequest {
        fn extract_access_key_and_region_from_aws(&self) -> ProxyResult<(&str, &str)> {
            let auth = self.headers().get(AUTHORIZATION).ok_or_else(|| {
                ProxyError::InvalidAuthorizationHeader("Missing authorization header".into())
            })?;
            let auth_str = auth.to_str().map_err(|_| {
                ProxyError::InvalidAuthorizationHeader(format!(
                    "Malformed authorization header, only visible ASCII chars allowed. \
                The authorization header: {auth:#?}"
                ))
            })?;
            extract_aws_access_key_and_region_from_auth_header(auth_str)
        }

        async fn sign_with_aws_sigv4_params(
            mut self,
            params: &AwsSigv4SignParams<'_>,
        ) -> ProxyResult<Self> {
            // save checksum before signing
            let checksum = self
                .headers()
                .get(X_AMZ_CONTENT_SHA_256)
                .ex("X_AMZ_CONTENT_SHA_256 should be set")
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
            let bytes = body::to_bytes(b).await.ex("body to bytes should work");
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
                .build()
                .ex("build signing_params should work");
            let signable_request = SignableRequest::from(&byte_req);
            let (signing_instructions, _signature) = sign(signable_request, &signing_params)
                .ex("sign should work")
                .into_parts();
            signing_instructions.apply_to_request(&mut byte_req);

            // restore body from bytes
            let (p, b) = byte_req.into_parts();
            self = Self::from_parts(p, Body::from(b));

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
                    (not a valid AMZ sigV4 authorization header): {auth_str}"
                ))
            })?
            .1;
        let region = split.nth(1).ok_or_else(|| {
            ProxyError::InvalidAuthorizationHeader(format!(
                "Malformed authorization header found when extract region\
                (not a valid AMZ sigV4 authorization header): {auth_str}"
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

    impl<'a> AwsSigv4SignParams<'a> {
        pub const fn new_with(account: &'a AwsAccount, service: &'a str, region: &'a str) -> Self {
            AwsSigv4SignParams {
                account,
                service,
                region,
            }
        }
    }

    #[cfg(test)]
    mod test {
        use crate::signature::aws::extract_aws_access_key_and_region_from_auth_header;

        #[test]
        fn test_extract_aws_access_key_and_region_from_auth_header() {
            let (key, region) = extract_aws_access_key_and_region_from_auth_header(
                "AWS4-HMAC-SHA256 Credential=AKPSSVCSPROXYDEV/20221012/cn-northwest-1/s3/aws4_request ..."
            ).unwrap();
            assert_eq!(key, "AKPSSVCSPROXYDEV");
            assert_eq!(region, "cn-northwest-1");
        }
    }
}

#[cfg(feature = "tencent-signature")]
pub mod tencent {
    use crate::{error::ProxyResult, type_alias::HttpRequest};

    pub trait TencentSig {
        fn extract_access_key_and_region_from_tencent(&self) -> ProxyResult<(&str, &str)>;
    }

    impl TencentSig for HttpRequest {
        fn extract_access_key_and_region_from_tencent(&self) -> ProxyResult<(&str, &str)> {
            todo!("extract access key and region from self (HttpRequest)")
        }
    }
}

#[allow(unused)]
pub fn split_to_base_and_account_code(access_key: &str) -> ProxyResult<(&str, &str)> {
    // TODO: split_to_base_and_account_code
    todo!("later")
}

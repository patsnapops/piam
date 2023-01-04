use piam_core::{input::Input, type_alias::HttpRequest};

use crate::{config::HostDomains, error::ParserResult, input::ObjectStorageInput};

impl Input for ObjectStorageInput {}

impl ObjectStorageInput {
    pub fn from(req: &HttpRequest, config: &HostDomains) -> ParserResult<Self> {
        #[cfg(feature = "cos-parser")]
        {
            // sample: "user-agent": "aws-sdk-rust/0.52.0 os/macos lang/rust/1.66.0"
            // sample: "user-agent": "cos-go-sdk-v5/0.7.35"
            let from_cos_sdk = match req.headers().get(http::header::USER_AGENT) {
                None => false,
                Some(hv) => hv
                    .to_str()
                    .map_err(|_| {
                        crate::error::ParserError::MalformedProtocol(
                            "user-agent must only contains visible ASCII chars".to_string(),
                        )
                    })?
                    .starts_with("cos"),
            };
            if from_cos_sdk {
                return Self::from_cos(req, config);
            }
        }
        Self::from_s3(req, config)
    }
}

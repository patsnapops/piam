use busylib::prelude::EnhancedExpect;
use http::{header::HOST, Method};
use piam_core::type_alias::HttpRequest;
use serde::Deserialize;

use crate::{
    config::HostDomains,
    error::{parse_error, ParserError, ParserResult},
    input::ObjectStorageInput,
};

#[derive(Debug, Deserialize)]
struct Query {
    #[serde(rename = "list-type")]
    list_type: Option<i32>,
    tagging: Option<String>,
    notification: Option<String>,
    uploads: Option<String>,
    #[serde(rename = "uploadId")]
    upload_id: Option<String>,
}

impl Query {
    fn has_list_type(&self) -> bool {
        self.list_type.is_some()
    }

    fn has_tagging(&self) -> bool {
        self.tagging.is_some()
    }

    fn has_uploads(&self) -> bool {
        self.uploads.is_some()
    }

    fn has_upload_id(&self) -> bool {
        self.upload_id.is_some()
    }

    fn has_notification(&self) -> bool {
        self.notification.is_some()
    }
}

impl ObjectStorageInput {
    pub fn from_s3(req: &HttpRequest, config: &HostDomains) -> ParserResult<Self> {
        let host = req
            .headers()
            .get(HOST)
            .ok_or_else(|| ParserError::MalformedProtocol("host missing in headers".to_string()))?
            .to_str()
            .map_err(|_| {
                ParserError::MalformedProtocol(
                    "host must only contains visible ASCII chars".to_string(),
                )
            })?;
        let proxy_host = config.find_proxy_host(host)?;

        if host == proxy_host && *req.method() == Method::GET {
            return Ok(ObjectStorageInput::ListBuckets);
        }

        let bucket = Self::get_bucket_name(host, proxy_host)?;
        let query = serde_urlencoded::from_str(req.uri().query().unwrap_or_default())
            .ex("query to form should work");

        if req.uri().path() == "/" {
            Self::parse_bucket_operations(req, bucket, &query)
        } else {
            Self::parse_object_operations(req, bucket, &query)
        }
    }

    fn get_bucket_name(host: &str, proxy_host: &str) -> ParserResult<String> {
        host.strip_suffix(&format!(".{}", proxy_host))
            .ok_or_else(|| {
                ParserError::OperationNotSupported(
                    "ListBuckets not supported due to uni-key feature".into(),
                )
            })
            .map(|s| s.to_string())
    }

    fn parse_bucket_operations(
        req: &HttpRequest,
        bucket: String,
        query: &Query,
    ) -> Result<ObjectStorageInput, ParserError> {
        use ObjectStorageInput::*;
        if query.has_list_type() && req.method() == Method::GET {
            Ok(ListObjects { bucket })
        } else if query.has_tagging() {
            match *req.method() {
                Method::GET => Ok(GetBucketTagging { bucket }),
                Method::PUT => Ok(PutBucketTagging { bucket }),
                Method::DELETE => Ok(DeleteBucketTagging { bucket }),
                _ => parse_error("unknown bucket tagging operation", req),
            }
        } else if query.has_tagging() {
            match *req.method() {
                Method::GET => Ok(ListMultiPartUploads { bucket }),
                _ => parse_error("unknown bucket uploads operation", req),
            }
        } else if query.has_notification() {
            match *req.method() {
                Method::GET => Ok(GetBucketNotificationConfiguration { bucket }),
                Method::PUT => Ok(PutBucketNotificationConfiguration { bucket }),
                _ => parse_error("unknown bucket notification operation", req),
            }
        } else {
            match *req.method() {
                // This is a special case for ListObjectsV1, which is not recommended by AWS
                Method::GET => Ok(ListObjects { bucket }),
                Method::PUT => Ok(CreateBucket { bucket }),
                Method::HEAD => Ok(HeadBucket { bucket }),
                Method::DELETE => Ok(DeleteBucket { bucket }),
                _ => parse_error("unknown bucket operation", req),
            }
        }
    }

    fn parse_object_operations(
        req: &HttpRequest,
        bucket: String,
        query: &Query,
    ) -> Result<ObjectStorageInput, ParserError> {
        use ObjectStorageInput::*;
        let key = req.uri().path()[1..].to_string();
        if query.has_uploads() {
            match *req.method() {
                Method::POST => Ok(CreateMultipartUpload { bucket, key }),
                _ => parse_error("unknown object upload operation", req),
            }
        } else if query.has_upload_id() {
            match *req.method() {
                Method::GET => Ok(ListParts { bucket, key }),
                Method::PUT => Ok(UploadPart { bucket, key }),
                Method::POST => Ok(CompleteMultipartUpload { bucket, key }),
                Method::DELETE => Ok(AbortMultipartUpload { bucket, key }),
                _ => parse_error("unknown object upload operation", req),
            }
        } else {
            match *req.method() {
                Method::GET => Ok(GetObject { bucket, key }),
                Method::PUT => match req.headers().get("x-amz-copy-source") {
                    Some(value) => {
                        let copy_source = value
                            .to_str()
                            .map_err(|_| {
                                ParserError::MalformedProtocol(
                                    "copy_source must only contains visible ASCII chars"
                                        .to_string(),
                                )
                            })?
                            .to_string();
                        Ok(CopyObject {
                            bucket,
                            key,
                            copy_source,
                        })
                    }
                    None => Ok(PutObject { bucket, key }),
                },
                Method::HEAD => Ok(HeadObject { bucket, key }),
                Method::DELETE => Ok(DeleteObject { bucket, key }),
                _ => parse_error("unknown object operation", req),
            }
        }
    }
}

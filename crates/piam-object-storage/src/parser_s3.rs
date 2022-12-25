use std::fmt;

use http::{header::HOST, Method};
use piam_core::{input::Input, type_alias::HttpRequest};
use serde::{Deserialize, Serialize};

use crate::input::ObjectStorageInput;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3HostDomains {
    pub domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Form {
    #[serde(rename = "list-type")]
    list_type: Option<i32>,
    tagging: Option<String>,
    notification: Option<String>,
    uploads: Option<String>,
    #[serde(rename = "uploadId")]
    upload_id: Option<String>,
}

impl S3HostDomains {
    pub fn find_proxy_host(&self, host: &str) -> ParserResult<&str> {
        let s = self
            .domains
            .iter()
            .find(|&v| host.ends_with(v))
            .ok_or_else(|| {
                ParserError::InvalidEndpoint(format!(
                    "'{}' is not ending with a valid piam s3 proxy endpoint",
                    host
                ))
            })?;
        Ok(s)
    }
}

pub enum ParserError {
    OperationNotSupported(String),
    InvalidEndpoint(String),
    Internal(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::OperationNotSupported(msg) => write!(f, "OperationNotSupported: {}", msg),
            ParserError::InvalidEndpoint(msg) => write!(f, "InvalidEndpoint: {}", msg),
            ParserError::Internal(msg) => write!(f, "Internal: {}", msg),
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

impl Input for ObjectStorageInput {}

impl ObjectStorageInput {
    pub fn from_s3(req: &HttpRequest, config: &S3HostDomains) -> ParserResult<Self> {
        use ObjectStorageInput::*;
        let path = req.uri().path();
        let method = req.method();
        let headers = req.headers();
        let host = headers.get(HOST).unwrap().to_str().unwrap();
        let proxy_host = config.find_proxy_host(host)?;
        let bucket = host
            .strip_suffix(&format!(".{}", proxy_host))
            .ok_or_else(|| {
                ParserError::OperationNotSupported(
                    "ListBuckets not supported due to uni-key feature".into(),
                )
            })?
            .to_string();
        let query = req.uri().query().unwrap_or_default();
        let form: Form = serde_urlencoded::from_str(query).unwrap();

        let has_list_type = form.list_type.is_some();
        let has_tagging = form.tagging.is_some();
        let has_uploads = form.uploads.is_some();
        let has_upload_id = form.upload_id.is_some();
        let has_notification = form.notification.is_some();

        if path == "/" {
            // bucket operations
            if has_list_type && *method == Method::GET {
                Ok(ListObjects { bucket })
            } else if has_tagging {
                match *method {
                    Method::GET => Ok(GetBucketTagging { bucket }),
                    Method::PUT => Ok(PutBucketTagging { bucket }),
                    Method::DELETE => Ok(DeleteBucketTagging { bucket }),
                    _ => parse_error("unknown bucket tagging operation", req),
                }
            } else if has_uploads {
                match *method {
                    Method::GET => Ok(ListMultiPartUploads { bucket }),
                    _ => parse_error("unknown bucket uploads operation", req),
                }
            } else if has_notification {
                match *method {
                    Method::GET => Ok(GetBucketNotificationConfiguration { bucket }),
                    Method::PUT => Ok(PutBucketNotificationConfiguration { bucket }),
                    _ => parse_error("unknown bucket notification operation", req),
                }
            } else if host == proxy_host && *method == Method::GET {
                Ok(ListBuckets)
            } else {
                match *method {
                    // This is a special case for ListObjectsV1, which is not recommended by AWS
                    Method::GET => Ok(ListObjects { bucket }),
                    Method::PUT => Ok(CreateBucket { bucket }),
                    Method::HEAD => Ok(HeadBucket { bucket }),
                    Method::DELETE => Ok(DeleteBucket { bucket }),
                    _ => parse_error("unknown bucket operation", req),
                }
            }
        } else {
            // object operations
            let key = path[1..].to_string();
            if has_uploads {
                match *method {
                    Method::POST => Ok(CreateMultipartUpload { bucket, key }),
                    _ => parse_error("unknown object upload operation", req),
                }
            } else if has_upload_id {
                match *method {
                    Method::GET => Ok(ListParts { bucket, key }),
                    Method::PUT => Ok(UploadPart { bucket, key }),
                    Method::POST => Ok(CompleteMultipartUpload { bucket, key }),
                    Method::DELETE => Ok(AbortMultipartUpload { bucket, key }),
                    _ => parse_error("unknown object upload operation", req),
                }
            } else {
                match *method {
                    Method::GET => Ok(GetObject { bucket, key }),
                    Method::PUT => match headers.get("x-amz-copy-source") {
                        Some(value) => {
                            let copy_source = value.to_str().unwrap().to_string();
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
}

fn parse_error(msg: &str, req: &HttpRequest) -> ParserResult<ObjectStorageInput> {
    let uri = req.uri().to_string();
    let method = req.method().to_string();
    let headers = req.headers().to_owned();
    Err(ParserError::OperationNotSupported(format!(
        "{} uri: {} method: {} headers: {:#?} ",
        msg, uri, method, headers
    )))
}

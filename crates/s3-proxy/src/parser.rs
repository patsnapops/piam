use http::{header::HOST, Method};
use piam_proxy_core::{
    error::{ProxyError, ProxyResult},
    input::Input,
    type_alias::HttpRequest,
};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::S3Config;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum S3Input {
    ListBuckets,
    CreateBucket {
        bucket: String,
    },
    HeadBucket {
        bucket: String,
    },
    DeleteBucket {
        bucket: String,
    },
    GetBucketTagging {
        bucket: String,
    },
    PutBucketTagging {
        bucket: String,
    },
    DeleteBucketTagging {
        bucket: String,
    },
    GetBucketNotificationConfiguration {
        bucket: String,
    },
    PutBucketNotificationConfiguration {
        bucket: String,
    },
    ListObjects {
        bucket: String,
    },
    ListMultiPartUploads {
        bucket: String,
    },
    GetObject {
        bucket: String,
        key: String,
    },
    PutObject {
        bucket: String,
        key: String,
    },
    HeadObject {
        bucket: String,
        key: String,
    },
    DeleteObject {
        bucket: String,
        key: String,
    },
    CopyObject {
        bucket: String,
        key: String,
        copy_source: String,
    },
    CreateMultipartUpload {
        bucket: String,
        key: String,
    },
    UploadPart {
        bucket: String,
        key: String,
    },
    CompleteMultipartUpload {
        bucket: String,
        key: String,
    },
    ListParts {
        bucket: String,
        key: String,
    },
    AbortMultipartUpload {
        bucket: String,
        key: String,
    },
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ActionKind {
    ListBuckets,
    Bucket,
    Object,
}

impl S3Input {
    pub fn action(&self) -> String {
        // generate codes like `S3Input::ListBuckets => "ListBuckets".into()` by strum_macros::Display
        self.to_string()
    }

    pub fn action_kind(&self) -> ActionKind {
        use ActionKind::*;
        match self {
            Self::ListBuckets => ListBuckets,
            Self::CreateBucket { .. } => Bucket,
            Self::HeadBucket { .. } => Bucket,
            Self::DeleteBucket { .. } => Bucket,
            Self::GetBucketTagging { .. } => Bucket,
            Self::PutBucketTagging { .. } => Bucket,
            Self::DeleteBucketTagging { .. } => Bucket,
            Self::GetBucketNotificationConfiguration { .. } => Bucket,
            Self::PutBucketNotificationConfiguration { .. } => Bucket,
            Self::ListObjects { .. } => Bucket,
            Self::ListMultiPartUploads { .. } => Bucket,
            Self::GetObject { .. } => Object,
            Self::PutObject { .. } => Object,
            Self::HeadObject { .. } => Object,
            Self::DeleteObject { .. } => Object,
            Self::CopyObject { .. } => Object,
            Self::CreateMultipartUpload { .. } => Object,
            Self::UploadPart { .. } => Object,
            Self::CompleteMultipartUpload { .. } => Object,
            Self::ListParts { .. } => Object,
            Self::AbortMultipartUpload { .. } => Object,
        }
    }

    /// # Panics
    ///
    /// If call this method on input that does not contains `bucket` field
    pub fn bucket(&self) -> &str {
        match self {
            Self::CreateBucket { bucket } => bucket,
            Self::HeadBucket { bucket } => bucket,
            Self::DeleteBucket { bucket } => bucket,
            Self::GetBucketTagging { bucket } => bucket,
            Self::PutBucketTagging { bucket } => bucket,
            Self::DeleteBucketTagging { bucket } => bucket,
            Self::ListObjects { bucket } => bucket,
            Self::ListMultiPartUploads { bucket } => bucket,
            Self::GetBucketNotificationConfiguration { bucket, .. } => bucket,
            Self::PutBucketNotificationConfiguration { bucket, .. } => bucket,
            Self::GetObject { bucket, .. } => bucket,
            Self::PutObject { bucket, .. } => bucket,
            Self::HeadObject { bucket, .. } => bucket,
            Self::DeleteObject { bucket, .. } => bucket,
            Self::CopyObject { bucket, .. } => bucket,
            Self::CreateMultipartUpload { bucket, .. } => bucket,
            Self::UploadPart { bucket, .. } => bucket,
            Self::CompleteMultipartUpload { bucket, .. } => bucket,
            Self::ListParts { bucket, .. } => bucket,
            Self::AbortMultipartUpload { bucket, .. } => bucket,
            _ => panic!("{} has no bucket", self.action()),
        }
    }

    /// # Panics
    ///
    /// If call this method on input that does not contains `key` field
    pub fn key(&self) -> &str {
        match self {
            Self::GetObject { key, .. } => key,
            Self::PutObject { key, .. } => key,
            Self::HeadObject { key, .. } => key,
            Self::DeleteObject { key, .. } => key,
            Self::CopyObject { key, .. } => key,
            Self::CreateMultipartUpload { key, .. } => key,
            Self::UploadPart { key, .. } => key,
            Self::CompleteMultipartUpload { key, .. } => key,
            Self::ListParts { key, .. } => key,
            Self::AbortMultipartUpload { key, .. } => key,
            _ => panic!("{} has no object", self.action()),
        }
    }
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

impl Input for S3Input {
    type Config = S3Config;

    fn from_http(req: &HttpRequest, config: Option<&S3Config>) -> ProxyResult<Self> {
        use S3Input::*;
        let path = req.uri().path();
        let method = req.method();
        let headers = req.headers();
        let host = headers.get(HOST).unwrap().to_str().unwrap();
        let config = config.expect("s3_config should be set");
        let proxy_host = config.find_proxy_host(host)?;
        let bucket = host
            .strip_suffix(&format!(".{}", proxy_host))
            .ok_or_else(|| {
                ProxyError::OperationNotSupported(
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

fn parse_error(msg: &str, req: &HttpRequest) -> ProxyResult<S3Input> {
    let uri = req.uri().to_string();
    let method = req.method().to_string();
    let headers = req.headers().to_owned();
    Err(ProxyError::OperationNotSupported(format!(
        "{} uri: {} method: {} headers: {:#?} ",
        msg, uri, method, headers
    )))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum S3Output {}

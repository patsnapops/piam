use anyhow::{anyhow, Result};
use http::{header::HOST, Method};
use log::error;
use piam_proxy_core::{input::Input, type_alias::HttpRequest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::Display;

use crate::config::S3_CONFIG;
use crate::error::InputOperationError;

#[derive(Debug)]
pub enum S3IoKind {
    Input(S3Input),
    Output(S3Output),
}

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

#[derive(Debug)]
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
        match self {
            S3Input::ListBuckets => ActionKind::ListBuckets,
            S3Input::CreateBucket { .. } => ActionKind::Bucket,
            S3Input::HeadBucket { .. } => ActionKind::Bucket,
            S3Input::DeleteBucket { .. } => ActionKind::Bucket,
            S3Input::GetBucketTagging { .. } => ActionKind::Bucket,
            S3Input::PutBucketTagging { .. } => ActionKind::Bucket,
            S3Input::DeleteBucketTagging { .. } => ActionKind::Bucket,
            S3Input::GetBucketNotificationConfiguration { .. } => ActionKind::Bucket,
            S3Input::PutBucketNotificationConfiguration { .. } => ActionKind::Bucket,
            S3Input::ListObjects { .. } => ActionKind::Bucket,
            S3Input::ListMultiPartUploads { .. } => ActionKind::Bucket,
            S3Input::GetObject { .. } => ActionKind::Object,
            S3Input::PutObject { .. } => ActionKind::Object,
            S3Input::HeadObject { .. } => ActionKind::Object,
            S3Input::DeleteObject { .. } => ActionKind::Object,
            S3Input::CopyObject { .. } => ActionKind::Object,
            S3Input::CreateMultipartUpload { .. } => ActionKind::Object,
            S3Input::UploadPart { .. } => ActionKind::Object,
            S3Input::CompleteMultipartUpload { .. } => ActionKind::Object,
            S3Input::ListParts { .. } => ActionKind::Object,
            S3Input::AbortMultipartUpload { .. } => ActionKind::Object,
        }
    }

    pub fn bucket(&self) -> Result<&String, InputOperationError> {
        match self {
            S3Input::CreateBucket { bucket } => Ok(bucket),
            S3Input::HeadBucket { bucket } => Ok(bucket),
            S3Input::DeleteBucket { bucket } => Ok(bucket),
            S3Input::GetBucketTagging { bucket } => Ok(bucket),
            S3Input::PutBucketTagging { bucket } => Ok(bucket),
            S3Input::DeleteBucketTagging { bucket } => Ok(bucket),
            S3Input::ListObjects { bucket } => Ok(bucket),
            S3Input::ListMultiPartUploads { bucket } => Ok(bucket),
            S3Input::GetBucketNotificationConfiguration { bucket, .. } => Ok(bucket),
            S3Input::PutBucketNotificationConfiguration { bucket, .. } => Ok(bucket),
            S3Input::GetObject { bucket, .. } => Ok(bucket),
            S3Input::PutObject { bucket, .. } => Ok(bucket),
            S3Input::HeadObject { bucket, .. } => Ok(bucket),
            S3Input::DeleteObject { bucket, .. } => Ok(bucket),
            S3Input::CopyObject { bucket, .. } => Ok(bucket),
            S3Input::CreateMultipartUpload { bucket, .. } => Ok(bucket),
            S3Input::UploadPart { bucket, .. } => Ok(bucket),
            S3Input::CompleteMultipartUpload { bucket, .. } => Ok(bucket),
            S3Input::ListParts { bucket, .. } => Ok(bucket),
            S3Input::AbortMultipartUpload { bucket, .. } => Ok(bucket),
            _ => Err(InputOperationError::InvalidBucketOp(self.action())),
        }
    }

    pub fn key(&self) -> Result<&String, InputOperationError> {
        match self {
            S3Input::GetObject { key, .. } => Ok(key),
            S3Input::PutObject { key, .. } => Ok(key),
            S3Input::HeadObject { key, .. } => Ok(key),
            S3Input::DeleteObject { key, .. } => Ok(key),
            S3Input::CopyObject { key, .. } => Ok(key),
            S3Input::CreateMultipartUpload { key, .. } => Ok(key),
            S3Input::UploadPart { key, .. } => Ok(key),
            S3Input::CompleteMultipartUpload { key, .. } => Ok(key),
            S3Input::ListParts { key, .. } => Ok(key),
            S3Input::AbortMultipartUpload { key, .. } => Ok(key),
            _ => Err(InputOperationError::InvalidObjectOp(self.action())),
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
    fn from_http(req: &HttpRequest) -> Result<Self> {
        let proxy_host = &S3_CONFIG.load().proxy_host;
        let path = req.uri().path();
        let method = req.method();
        let headers = req.headers();
        let host = headers.get(HOST).unwrap().to_str().unwrap();
        let bucket = host.replace(format!(".{}", proxy_host).as_str(), "");
        let query = req.uri().query().unwrap_or_default();
        let form: Form = serde_urlencoded::from_str(query).unwrap();

        let has_list_type = form.list_type.is_some();
        let has_tagging = form.tagging.is_some();
        let has_uploads = form.uploads.is_some();
        let has_upload_id = form.upload_id.is_some();
        let has_notification = form.notification.is_some();
        let empty_query = query.is_empty();

        if path == "/" {
            // bucket operations
            if empty_query {
                if host == proxy_host && *method == Method::GET {
                    Ok(S3Input::ListBuckets)
                } else {
                    match *method {
                        // This is a special case for ListObjectsV1, which is not recommended by AWS
                        Method::GET => Ok(S3Input::ListObjects { bucket }),
                        Method::PUT => Ok(S3Input::CreateBucket { bucket }),
                        Method::HEAD => Ok(S3Input::HeadBucket { bucket }),
                        Method::DELETE => Ok(S3Input::DeleteBucket { bucket }),
                        _ => parse_error("unknown bucket operation", req),
                    }
                }
            } else if has_list_type && *method == Method::GET {
                Ok(S3Input::ListObjects { bucket })
            } else if has_tagging {
                match *method {
                    Method::GET => Ok(S3Input::GetBucketTagging { bucket }),
                    Method::PUT => Ok(S3Input::PutBucketTagging { bucket }),
                    Method::DELETE => Ok(S3Input::DeleteBucketTagging { bucket }),
                    _ => parse_error("unknown bucket tagging operation", req),
                }
            } else if has_uploads {
                match *method {
                    Method::GET => Ok(S3Input::ListMultiPartUploads { bucket }),
                    _ => parse_error("unknown bucket uploads operation", req),
                }
            } else if has_notification {
                match *method {
                    Method::GET => Ok(S3Input::GetBucketNotificationConfiguration { bucket }),
                    Method::PUT => Ok(S3Input::PutBucketNotificationConfiguration { bucket }),
                    _ => parse_error("unknown bucket notification operation", req),
                }
            } else {
                parse_error("bucket operation not yet supported", req)
            }
        } else {
            // object operations
            let key = path[1..].to_string();
            if empty_query || (!has_tagging && !has_uploads && !has_upload_id) {
                match *method {
                    Method::GET => Ok(S3Input::GetObject { bucket, key }),
                    Method::PUT => match headers.get("x-amz-copy-source") {
                        Some(value) => {
                            let copy_source = value.to_str().unwrap().to_string();
                            Ok(S3Input::CopyObject {
                                bucket,
                                key,
                                copy_source,
                            })
                        }
                        None => Ok(S3Input::PutObject { bucket, key }),
                    },
                    Method::HEAD => Ok(S3Input::HeadObject { bucket, key }),
                    Method::DELETE => Ok(S3Input::DeleteObject { bucket, key }),
                    _ => parse_error("unknown object operation", req),
                }
            } else if has_uploads {
                match *method {
                    Method::POST => Ok(S3Input::CreateMultipartUpload { bucket, key }),
                    _ => parse_error("unknown object upload operation", req),
                }
            } else if has_upload_id {
                match *method {
                    Method::GET => Ok(S3Input::ListParts { bucket, key }),
                    Method::PUT => Ok(S3Input::UploadPart { bucket, key }),
                    Method::POST => Ok(S3Input::CompleteMultipartUpload { bucket, key }),
                    Method::DELETE => Ok(S3Input::AbortMultipartUpload { bucket, key }),
                    _ => parse_error("unknown object upload operation", req),
                }
            } else {
                parse_error("object operation not yet supported", req)
            }
        }
    }
}

fn parse_error(msg: &str, req: &HttpRequest) -> Result<S3Input> {
    error!("{}: {:#?}", msg, req);
    Err(anyhow!(""))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum S3Output {}

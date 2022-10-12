use anyhow::{anyhow, Result};
use http::{header::HOST, Method};
use log::error;
use piam_proxy_core::{input::Input, type_alias::HttpRequest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::config::S3_CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub enum S3IoKind {
    Input(S3Input),
    Output(S3Output),
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum S3Input {
    #[default]
    Any,
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

impl S3Input {
    pub fn action(&self) -> String {
        match self {
            S3Input::Any => "Any".into(),
            S3Input::ListBuckets => "ListBuckets".into(),
            S3Input::CreateBucket { .. } => "CreateBucket".into(),
            S3Input::HeadBucket { .. } => "HeadBucket".into(),
            S3Input::DeleteBucket { .. } => "DeleteBucket".into(),
            S3Input::GetBucketTagging { .. } => "GetBucketTagging".into(),
            S3Input::PutBucketTagging { .. } => "PutBucketTagging".into(),
            S3Input::DeleteBucketTagging { .. } => "DeleteBucketTagging".into(),
            S3Input::ListObjects { .. } => "ListObjects".into(),
            S3Input::ListMultiPartUploads { .. } => "ListMultiPartUploads".into(),
            S3Input::GetObject { .. } => "GetObject".into(),
            S3Input::PutObject { .. } => "PutObject".into(),
            S3Input::HeadObject { .. } => "HeadObject".into(),
            S3Input::DeleteObject { .. } => "DeleteObject".into(),
            S3Input::CopyObject { .. } => "CopyObject".into(),
            S3Input::CreateMultipartUpload { .. } => "CreateMultipartUpload".into(),
            S3Input::UploadPart { .. } => "UploadPart".into(),
            S3Input::CompleteMultipartUpload { .. } => "CompleteMultipartUpload".into(),
            S3Input::ListParts { .. } => "ListParts".into(),
            S3Input::AbortMultipartUpload { .. } => "AbortMultipartUpload".into(),
        }
    }

    pub fn bucket(&self) -> &String {
        match self {
            S3Input::CreateBucket { bucket } => bucket,
            S3Input::HeadBucket { bucket } => bucket,
            S3Input::DeleteBucket { bucket } => bucket,
            S3Input::GetBucketTagging { bucket } => bucket,
            S3Input::PutBucketTagging { bucket } => bucket,
            S3Input::DeleteBucketTagging { bucket } => bucket,
            S3Input::ListObjects { bucket } => bucket,
            S3Input::ListMultiPartUploads { bucket } => bucket,
            S3Input::GetObject { bucket, .. } => bucket,
            S3Input::PutObject { bucket, .. } => bucket,
            S3Input::HeadObject { bucket, .. } => bucket,
            S3Input::DeleteObject { bucket, .. } => bucket,
            S3Input::CopyObject { bucket, .. } => bucket,
            S3Input::CreateMultipartUpload { bucket, .. } => bucket,
            S3Input::UploadPart { bucket, .. } => bucket,
            S3Input::CompleteMultipartUpload { bucket, .. } => bucket,
            S3Input::ListParts { bucket, .. } => bucket,
            S3Input::AbortMultipartUpload { bucket, .. } => bucket,
            _ => panic!("Unexpected S3InputType"),
        }
    }

    pub fn key(&self) -> &String {
        match self {
            S3Input::GetObject { key, .. } => key,
            S3Input::PutObject { key, .. } => key,
            S3Input::HeadObject { key, .. } => key,
            S3Input::DeleteObject { key, .. } => key,
            S3Input::CopyObject { key, .. } => key,
            S3Input::CreateMultipartUpload { key, .. } => key,
            S3Input::UploadPart { key, .. } => key,
            S3Input::CompleteMultipartUpload { key, .. } => key,
            S3Input::ListParts { key, .. } => key,
            S3Input::AbortMultipartUpload { key, .. } => key,
            _ => panic!("Unexpected S3InputType"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Form {
    #[serde(rename = "list-type")]
    list_type: Option<i32>,
    tagging: Option<String>,
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
        let empty_query = query.is_empty();

        if path == "/" {
            // bucket operations
            if empty_query {
                if host == proxy_host && *method == Method::GET {
                    Ok(S3Input::ListBuckets)
                } else {
                    match *method {
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

impl Serialize for S3Input {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Any => serializer.serialize_str("*"),
            Self::ListBuckets => serializer.serialize_str("ListBuckets"),
            Self::CreateBucket { .. } => serializer.serialize_str("CreateBucket"),
            Self::HeadBucket { .. } => serializer.serialize_str("HeadBucket"),
            Self::DeleteBucket { .. } => serializer.serialize_str("DeleteBucket"),
            Self::GetBucketTagging { .. } => serializer.serialize_str("GetBucketTagging"),
            Self::PutBucketTagging { .. } => serializer.serialize_str("PutBucketTagging"),
            Self::DeleteBucketTagging { .. } => serializer.serialize_str("DeleteBucketTagging"),
            Self::ListObjects { .. } => serializer.serialize_str("ListObjects"),
            Self::ListMultiPartUploads { .. } => serializer.serialize_str("ListMultiPartUploads"),
            Self::GetObject { .. } => serializer.serialize_str("GetObject"),
            Self::PutObject { .. } => serializer.serialize_str("PutObject"),
            Self::HeadObject { .. } => serializer.serialize_str("HeadObject"),
            Self::DeleteObject { .. } => serializer.serialize_str("DeleteObject"),
            Self::CopyObject { .. } => serializer.serialize_str("CopyObject"),
            Self::CreateMultipartUpload { .. } => serializer.serialize_str("CreateMultipartUpload"),
            Self::UploadPart { .. } => serializer.serialize_str("UploadPart"),
            Self::CompleteMultipartUpload { .. } => {
                serializer.serialize_str("CompleteMultipartUpload")
            }
            Self::ListParts { .. } => serializer.serialize_str("ListParts"),
            Self::AbortMultipartUpload { .. } => serializer.serialize_str("AbortMultipartUpload"),
        }
    }
}

impl<'de> Deserialize<'de> for S3Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "*" => Ok(Self::Any),
            "ListBuckets" => Ok(Self::ListBuckets),
            "CreateBucket" => Ok(Self::CreateBucket {
                bucket: String::new(),
            }),
            "HeadBucket" => Ok(Self::HeadBucket {
                bucket: String::new(),
            }),
            "DeleteBucket" => Ok(Self::DeleteBucket {
                bucket: String::new(),
            }),
            "GetBucketTagging" => Ok(Self::GetBucketTagging {
                bucket: String::new(),
            }),
            "PutBucketTagging" => Ok(Self::PutBucketTagging {
                bucket: String::new(),
            }),
            "DeleteBucketTagging" => Ok(Self::DeleteBucketTagging {
                bucket: String::new(),
            }),
            "ListObjects" => Ok(Self::ListObjects {
                bucket: String::new(),
            }),
            "ListMultiPartUploads" => Ok(Self::ListMultiPartUploads {
                bucket: String::new(),
            }),
            "GetObject" => Ok(Self::GetObject {
                bucket: String::new(),
                key: String::new(),
            }),
            "PutObject" => Ok(Self::PutObject {
                bucket: String::new(),
                key: String::new(),
            }),
            "HeadObject" => Ok(Self::HeadObject {
                bucket: String::new(),
                key: String::new(),
            }),
            "DeleteObject" => Ok(Self::DeleteObject {
                bucket: String::new(),
                key: String::new(),
            }),
            "CopyObject" => Ok(Self::CopyObject {
                bucket: String::new(),
                key: String::new(),
                copy_source: String::new(),
            }),
            "CreateMultipartUpload" => Ok(Self::CreateMultipartUpload {
                bucket: String::new(),
                key: String::new(),
            }),
            "UploadPart" => Ok(Self::UploadPart {
                bucket: String::new(),
                key: String::new(),
            }),
            "CompleteMultipartUpload" => Ok(Self::CompleteMultipartUpload {
                bucket: String::new(),
                key: String::new(),
            }),
            "ListParts" => Ok(Self::ListParts {
                bucket: String::new(),
                key: String::new(),
            }),
            "AbortMultipartUpload" => Ok(Self::AbortMultipartUpload {
                bucket: String::new(),
                key: String::new(),
            }),
            _ => panic!("Unknown S3InputType: {}", s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum S3Output {}

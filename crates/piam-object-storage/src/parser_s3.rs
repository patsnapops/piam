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
struct Form {
    #[serde(rename = "list-type")]
    list_type: Option<i32>,
    tagging: Option<String>,
    notification: Option<String>,
    uploads: Option<String>,
    #[serde(rename = "uploadId")]
    upload_id: Option<String>,
}

impl ObjectStorageInput {
    pub fn from_s3(req: &HttpRequest, config: &HostDomains) -> ParserResult<Self> {
        use ObjectStorageInput::*;
        let path = req.uri().path();
        let method = req.method();
        let headers = req.headers();
        let host = headers
            .get(HOST)
            .ok_or_else(|| ParserError::MalformedProtocol("host missing in headers".to_string()))?
            .to_str()
            .map_err(|_| {
                ParserError::MalformedProtocol(
                    "host must only contains visible ASCII chars".to_string(),
                )
            })?;
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
        let form: Form = serde_urlencoded::from_str(query).ex("query to form should work");

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
                        None => {
                            // if the value of header "expect" is "100-continue",
                            // then the request is a chunked upload which is not supported currently
                            if let Some(expect) = headers.get("expect") {
                                if expect == "100-continue" {
                                    return parse_error(
                                        "chunked upload is not supported currently, \
                                        troubleshooting: http://ida.patsnap.info/piam/docs/user/s3/limitation",
                                        req,
                                    );
                                }
                            }
                            Ok(PutObject { bucket, key })
                        }
                    },
                    Method::HEAD => Ok(HeadObject { bucket, key }),
                    Method::DELETE => Ok(DeleteObject { bucket, key }),
                    _ => parse_error("unknown object operation", req),
                }
            }
        }
    }
}

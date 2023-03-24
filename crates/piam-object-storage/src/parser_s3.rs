use busylib::prelude::EnhancedExpect;
use http::{header::HOST, Method};
use piam_core::{input::InputAndRequest, type_alias::HttpRequest};
use serde::Deserialize;

use crate::{
    config::HostDomains,
    error::{parse_error, ParserError, ParserResult},
    input::{ObjectStorageInput, ObjectStorageInput::DeleteObjects},
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
    delete: Option<String>,
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

    fn has_delete(&self) -> bool {
        self.delete.is_some()
    }
}

impl ObjectStorageInput {
    pub async fn parse_s3(
        req: HttpRequest,
        config: &HostDomains,
    ) -> ParserResult<InputAndRequest<ObjectStorageInput>> {
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
            return Ok(InputAndRequest::new(ObjectStorageInput::ListBuckets, req));
        }

        let bucket = Self::get_bucket_name(host, proxy_host)?;
        let query: Query = serde_urlencoded::from_str(req.uri().query().unwrap_or_default())
            .ex("query to form should work");

        if req.uri().path() == "/" && !query.has_delete() {
            Self::parse_bucket_operations(req, bucket, &query)
        } else {
            Self::parse_object_operations(req, bucket, &query).await
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
        req: HttpRequest,
        bucket: String,
        query: &Query,
    ) -> Result<InputAndRequest<ObjectStorageInput>, ParserError> {
        use ObjectStorageInput::*;
        let input = if query.has_list_type() && req.method() == Method::GET {
            Ok(ListObjects { bucket })
        } else if query.has_tagging() {
            match *req.method() {
                Method::GET => Ok(GetBucketTagging { bucket }),
                Method::PUT => Ok(PutBucketTagging { bucket }),
                Method::DELETE => Ok(DeleteBucketTagging { bucket }),
                _ => parse_error("unknown bucket tagging operation", &req),
            }
        } else if query.has_tagging() {
            match *req.method() {
                Method::GET => Ok(ListMultiPartUploads { bucket }),
                _ => parse_error("unknown bucket uploads operation", &req),
            }
        } else if query.has_notification() {
            match *req.method() {
                Method::GET => Ok(GetBucketNotificationConfiguration { bucket }),
                Method::PUT => Ok(PutBucketNotificationConfiguration { bucket }),
                _ => parse_error("unknown bucket notification operation", &req),
            }
        } else {
            match *req.method() {
                // This is a special case for ListObjectsV1, which is not recommended by AWS
                Method::GET => Ok(ListObjects { bucket }),
                Method::PUT => Ok(CreateBucket { bucket }),
                Method::HEAD => Ok(HeadBucket { bucket }),
                Method::DELETE => Ok(DeleteBucket { bucket }),
                _ => parse_error("unknown bucket operation", &req),
            }
        }?;
        Ok(InputAndRequest::new(input, req))
    }

    async fn parse_object_operations(
        req: HttpRequest,
        bucket: String,
        query: &Query,
    ) -> Result<InputAndRequest<ObjectStorageInput>, ParserError> {
        use ObjectStorageInput::*;
        let key = req.uri().path()[1..].to_string();
        let input = if query.has_uploads() {
            match *req.method() {
                Method::POST => Ok(CreateMultipartUpload { bucket, key }),
                _ => parse_error("unknown object upload operation", &req),
            }
        } else if query.has_upload_id() {
            match *req.method() {
                Method::GET => Ok(ListParts { bucket, key }),
                Method::PUT => Ok(UploadPart { bucket, key }),
                Method::POST => Ok(CompleteMultipartUpload { bucket, key }),
                Method::DELETE => Ok(AbortMultipartUpload { bucket, key }),
                _ => parse_error("unknown object upload operation", &req),
            }
        } else if query.has_delete() {
            match *req.method() {
                Method::POST => {
                    return Self::parse_delete_objects(bucket, req).await;
                }
                _ => parse_error("unknown bucket delete operation", &req),
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
                _ => parse_error("unknown object operation", &req),
            }
        }?;
        Ok(InputAndRequest::new(input, req))
    }

    async fn parse_delete_objects(
        bucket: String,
        req: HttpRequest,
    ) -> Result<InputAndRequest<ObjectStorageInput>, ParserError> {
        #[derive(Debug, Deserialize)]
        struct S3DeleteObjects {
            #[serde(rename = "Object")]
            objects: Vec<S3Object>,
            #[allow(dead_code)]
            #[serde(rename = "VersionId")]
            version_id: Option<String>,
            #[allow(dead_code)]
            #[serde(rename = "Quiet")]
            quiet: Option<bool>,
        }
        #[derive(Debug, Deserialize)]
        struct S3Object {
            #[serde(rename = "Key")]
            key: String,
        }

        let (parts, body) = req.into_parts();
        let bytes = hyper::body::to_bytes(body)
            .await
            .map_err(|e| ParserError::Internal(format!("failed to read body of request: {}", e)))?;
        let xml = String::from_utf8(bytes.to_vec())
            .map_err(|_| ParserError::MalformedProtocol("body must be valid UTF-8".to_string()))?;
        let to_del: S3DeleteObjects = serde_xml_rs::from_str(&xml)
            .map_err(|e| ParserError::MalformedProtocol(format!("failed to parse xml: {}", e)))?;

        Ok(InputAndRequest::new(
            DeleteObjects {
                bucket,
                keys: to_del.objects.into_iter().map(|o| o.key).collect(),
            },
            HttpRequest::from_parts(parts, hyper::Body::from(bytes)),
        ))
    }
}

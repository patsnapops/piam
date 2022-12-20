use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display, Serialize, Deserialize)]
pub enum ObjectStorageInput {
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectStorageOutput {}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ActionKind {
    ListBuckets,
    Bucket,
    Object,
}

impl ObjectStorageInput {
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

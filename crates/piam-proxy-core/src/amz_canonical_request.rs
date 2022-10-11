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

# Changelog

All notable changes to this project will be documented in this file.

## [v0.20.2] - 2023-04-13

### Features

- Check policy conflicts when matching effects

## [v0.20.1] - 2023-04-13

### Bug Fixes

- Avoid key ambiguity

### Build

- Remove Cargo lock file from git

## [v0.20.0] - 2023-04-13

### Features

- Bring the keys in the object storage policy to the same level as the bucket

## [v0.19.0] - 2023-04-13

### Features

- Add region us-east-2
- Support DeleteObjects

## [v0.17.3] - 2023-04-13

### Bug Fixes

- Remove all possible panics from request.rs

## [v0.17.2] - 2023-04-13

### Bug Fixes

- Add a workaround due to unverified inconsistent behavior of the Tencent COS API

## [v0.17.0] - 2023-04-13

### Features

- Support special characters in url

## [v0.16.0] - 2023-04-13

### Features

- Allow specifying region for a bucket

## [v0.15.1] - 2023-04-13

### Bug Fixes

- Blocking issue when fetching config

## [v0.15.0] - 2023-04-13

### Features

- Make proxy can support multi-cloud signature extraction

## [v0.14.1] - 2023-04-13

### Features

- Handle various errors properly for s3 parser and adapt_path_style function

### Performance

- Optimize some functions to const fn

## [v0.14.0] - 2023-04-13

### Features

- Add prefilter for filtering group related content

## [v0.13.0] - 2023-04-13

### Features

- Rename piam-proxy-core to piam-core
- Split the proxy part of piam-core into a feature
- Separate proxy crate from core

## [v0.12.0] - 2023-04-13

### Features

- Log ip info of proxy when failed to get buckets for account
- Manually encrypt sensitive info for HTTP

## [v0.11.1] - 2023-04-13

### Bug Fixes

- Remove some expects

## [v0.11.0] - 2023-04-13

### Bug Fixes

- Replace all unwraps and expects

### Features

- When initializing, try to get new state until successful
- Add cidr whitelist support by introducing condition modeled policy

## [v0.10.1] - 2023-04-13

### Build

- Rename crate name of s3-proxy

## [v0.10.0] - 2023-04-13

### Features

- Support auto region finding for uni-key feature

## [v0.9.1] - 2023-04-13

### Bug Fixes

- Region typo for uni-key feature

### Features

- Add ak_id when failed to get buckets

## [v0.9.0] - 2023-04-13

### Features

- Support Tencent Cloud COS in a hard-coded way

## [v0.8.0] - 2023-04-13

### Features

- Add request_id for aws_xml_error_payload

## [v0.7.1] - 2023-04-13

### Bug Fixes

- Change panics to typed error InvalidEndpoint
- Typo in into_response

## [v0.7.0] - 2023-04-13

### Features

- Add trace info with CLUSTER_ENV

## [v0.6.0] - 2023-04-13

### Features

- Support multi accounts and regions

## [v0.5.1] - 2023-04-13

### Bug Fixes

- Make find_proxy_host find exact one host
- Logic flaws of extracting bucket

## [v0.5.0] - 2023-04-13

### Features

- Make error response compatible with s3 sdk, with additional PIAM error codes and messages

## [v0.4.1] - 2023-04-13

### Bug Fixes

- Matching bug introduced by ce04418e2b96e78f2b011fe60f21383785c09b30

## [v0.4.0] - 2023-04-13

### Features

- Support multiple proxy hosts

## [v0.3.0] - 2023-04-13

### Features

- Simplify yaml representation of principals and policies by changing data types of them

## [v0.2.1] - 2022-10-14

### Bug Fixes

- Parser error when request has unknown queries

## [v0.2.0] - 2022-10-12

### Features

- Add support for ListObjectsV1 due to business requirement

## [v0.1.0] - 2022-10-12

### Bug Fixes

- Return 403 when access key not provided in request

### Features

- Add basic s3 proxy

<!-- generated by git-cliff -->
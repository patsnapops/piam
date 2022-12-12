---
sidebar_position: 1
---

# 🚀 功能特性

## 支持多种维度的访问方式

### 多语言

支持 Go、Java、Python、Rust、Shell(cli `aws`) 等各种语言的 AWS SDK 与工具。

### 多云账号

使用一个 PIAM Uni Access Key 即可访问所有 Patsnap 账号下的所有桶，无需关心桶所在账号。

目前已添加:

| Account Code | Description   |
|--------------|---------------|
| 3977         | AWS 中国账号      |
| 7478         | AWS 美国账号      |
| 0066         | AWS 美国账号-数据处理 |
| 4258         | Tencent 全球账号  |

### 多云地区

使用一个 PIAM Uni Access Key 即可访问所有 Patsnap 账号下的所有桶，无需关心桶所在地区。

目前已添加:

| AWS            | Tencent     |
|----------------|-------------|
| us-east-1      | na-ashburn  |
| cn-northwest-1 | ap-shanghai |
| eu-central-1   |             |

### 多 Endpoint

支持使用不同的 Endpoint，用户请求将通过 OPS Infra 自动路由到合适的 Proxy 节点以**降低流量费用**。

目前已添加:

- http://local.s3-proxy.patsnap.info
- http://internal.s3-proxy.patsnap.info
- http://us-east-1.s3-proxy.patsnap.info
- http://na-ashburn.s3-proxy.patsnap.info
- http://cn-northwest-1.s3-proxy.patsnap.info
- http://ap-shanghai.s3-proxy.patsnap.info

### 多路径风格

已同时支持[虚拟托管式访问和路径类型访问](https://docs.aws.amazon.com/zh_cn/AmazonS3/latest/userguide/access-bucket-intro.html)，无特殊需求无需关心

## 支持的 API

:::info S3 API 列表

[https://docs.aws.amazon.com/zh_cn/AmazonS3/latest/API/API_Operations_Amazon_Simple_Storage_Service.html](https://docs.aws.amazon.com/zh_cn/AmazonS3/latest/API/API_Operations_Amazon_Simple_Storage_Service.html)

:::

### 对象

- GetObject
- PutObject
- HeadObject
- DeleteObject
- CopyObject
- CreateMultipartUpload
- UploadPart
- CompleteMultipartUpload
- ListParts
- AbortMultipartUpload

### 桶

- CreateBucket
- HeadBucket
- DeleteBucket
- GetBucketTagging
- PutBucketTagging
- DeleteBucketTagging
- GetBucketNotificationConfiguration
- PutBucketNotificationConfiguration
- ListObjects
- ListMultiPartUploads

### 特殊

- ListBuckets

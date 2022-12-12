---
sidebar_position: 2
---

# 🔌 接入指南

1. [获取权限与 Access Key](/docs/feedback/feedback.md)
2. 配置 AWS SDK/CLI 的 [Access Key](#access-key) 与 [Endpoint](#endpoint)
3. 访问对应资源

## Access Key

`ACCESS_KEY_ID` = `UNI_KEY` 必填 + `ACCOUNT_CODE` 可选

`Secret Key` = "optional" 可选

### 方式 1. Using the PIAM `UNI_KEY`

`ACCESS_KEY_ID` = `UNI_KEY`

### 方式 2. Using PIAM `UNI_KEY` with [`ACCOUNT_CODE`](feat#多云账号) as suffix

**仅在**需要 **ListBuckets** (列出某账号的某地区下的所有桶) 时使用这种格式的 Access Key。其余[18种操作](feat#支持的-api)都推荐使用第一种方式。

<details>
<summary>展开详情</summary>

`ACCESS_KEY_ID` = `UNI_KEY` + `ACCOUNT_CODE`

样例:

#### 列出 aws 7478 账号下美国地区的所有桶


`UNI_KEY` = "AKPSPERS03LJW0Z" `ACCOUNT_CODE` = "7478" `ACCESS_KEY_ID` = "AKPSPERS03LJW0Z**7478**"

`Region` = "us-east-1"

#### 列出 tencent global 账号下上海地区的所有桶

`UNI_KEY` = "AKPSPERS03LJW0Z" `ACCOUNT_CODE` = "4258" `ACCESS_KEY_ID` = "AKPSPERS03LJW0Z**4258**"

`REGION` = "ap-shanghai"

</details>

## Endpoint

任意 Endpoint 都可访问所有资源，建议使用合适的 Endpoint 以**降低流量费用**。

### 从 Kubernetes 集群内部访问 S3/COS

支持使用集群内部地址访问所有账号下的桶:
- http://internal.s3-proxy.patsnap.info

### 在 Kubernetes 集群外部访问 S3/COS

支持所有外部地址访问所有账号下的桶:
- http://us-east-1.s3-proxy.patsnap.info
- http://na-ashburn.s3-proxy.patsnap.info
- http://cn-northwest-1.s3-proxy.patsnap.info
- http://ap-shanghai.s3-proxy.patsnap.info
- http://local.s3-proxy.patsnap.info

**建议使用访问源就近的 Endpoint 以降低流量费用**

如程序部署在 AWS us-east-1 的 VPC 上，则使用 http://us-east-1.s3-proxy.patsnap.info

如在本地网络访问则使用 http://local.s3-proxy.patsnap.info

## 各语言配置方式

### Golang

to be added

### Java

```
BasicAWSCredentials piamCreds = new BasicAWSCredentials("`PIAM Uni Access Key`", "anything");
AmazonS3 s3 = AmazonS3ClientBuilder.standard()
        .withEndpointConfiguration(new AwsClientBuilder.EndpointConfiguration(
                "http://internal.s3-proxy.patsnap.info", Regions.CN_NORTHWEST_1.getName()
        ))
        // Please disable retries when not needed
        .withClientConfiguration(new ClientConfiguration().withMaxErrorRetry(0))
        // Please set chunked encoding to true
        .withChunkedEncodingDisabled(true)
        .withCredentials(new AWSStaticCredentialsProvider(piamCreds))
        .build();
```

### Python

### Rust

### Shell `aws` cli


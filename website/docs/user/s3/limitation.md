---
sidebar_position: 4
---

# 🚧 使用限制

- 一个 `PIAM Uni Key` 不会具有访问所有资源的权限，如缺，可联系 OPS 添加
- 未支持对象路径中包含特殊字符
- 未支持 HTTPS 

- Java SDK PutObject 时未支持分块传输模式

:::tip Workaround for Java SDK PutObject

初始化 S3 Client 时设定 `withChunkedEncodingDisabled` 参数为 true，样例：
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

:::

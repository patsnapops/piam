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
AmazonS3 s3 = AmazonS3ClientBuilder.standard()
            .withEndpointConfiguration(new AwsClientBuilder.EndpointConfiguration(
                    endpoint, Regions.CN_NORTHWEST_1.getName()
            ))
            .withClientConfiguration(
                    new ClientConfiguration()
                            // Disable retries when not needed for budget cutting
                            .withMaxErrorRetry(0)
                            // Set proper timeouts due for bandwidth limitation
                            .withClientExecutionTimeout(Integer.MAX_VALUE)
                            .withConnectionTimeout(Integer.MAX_VALUE)
                            .withSocketTimeout(Integer.MAX_VALUE)
                            .withRequestTimeout(Integer.MAX_VALUE)
            )
            // Chunked encoding MUST be disabled
            .withChunkedEncodingDisabled(true)
            .withCredentials(new AWSStaticCredentialsProvider(piamCreds))
            .build();
```

:::

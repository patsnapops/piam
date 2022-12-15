---
sidebar_position: 5
---

# 🐛 故障排除

PIAM S3 Proxy 的错误信息兼容了 AWS 标准，所有经过 Proxy 的报错都会在 AWS SDK/CLI 的错误堆栈中体现。

如报错信息中**未见到** "**PIAM**", "**[Patsnap S3 Proxy]**" 等字样: 
- 则说明该错误信息不来自 Proxy 本身，原因可能为:
  - 客户端未经过 Proxy 而直接向 **AWS** 发送了请求，应检查[SDK 配置](/docs/user/s3/usage)是否正确。
  - Patsnap 基础设施限制，如 `Your IP address is not allowed`，请调用链路是否通畅，联系 OPS。
  - 特殊的[使用限制](/docs/user/s3/limitation)。
  - ...

如报错信息中**包含** "**PIAM**", "**[Patsnap S3 Proxy]**" 等字样，可按下表排查原因:

## 错误类型

[//]: # (https://google-cloud.gitbook.io/api-design-guide/errors)

### 客户端错误

[//]: # (| Account Code | Description   |)

[//]: # (|--------------|---------------|)

[//]: # (| 3977         | AWS 中国账号      |)

[//]: # (| 7478         | AWS 美国账号      |)

[//]: # (| 0066         | AWS 美国账号-数据处理 |)

[//]: # (| 4258         | Tencent 全球账号  |)

### 服务端错误

[//]: # (错误信息里面放链接)

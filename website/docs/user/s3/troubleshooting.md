---
sidebar_position: 5
---

# 🐛 故障排除

S3 Proxy 的错误信息兼容了 AWS 标准。

凡所有经过 S3 Proxy 的报错都会在 SDK/CLI 的错误堆栈中体现

## 错误类型

[//]: # (https://google-cloud.gitbook.io/api-design-guide/errors)

### 客户端错误

| Account Code | Description   |
|--------------|---------------|
| 3977         | AWS 中国账号      |
| 7478         | AWS 美国账号      |
| 0066         | AWS 美国账号-数据处理 |
| 4258         | Tencent 全球账号  |

### 服务端错误

[//]: # (错误信息里面放链接)

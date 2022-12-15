---
sidebar_position: 1
---

# 概览

出于数据安全等[原因](/docs/developer/design/intro#背景)，部分场景云上资源需通过 PIAM Proxy 代理访问。这是一份供 Proxy 用户接入使用的手册。

:::tip 浏览手册

点击**左侧**列表进行章节跳转，**右侧**列表进行页内跳转。点击**绿色**超链接文本跳转到定义。

:::

[//]: # (## 友情链接)

[//]: # ()
[//]: # (- [S3对象同步、复制]&#40;http://todo.com&#41;)

[//]: # (- [模型发布]&#40;http://todo.com&#41;)

## 基本概念

在接入 PIAM Proxy 之前，以下是用户可能需要关注了解的概念：

- Access Key ID: 用于标识**用户身份**，可通过 AWS SDK 配置。

- Secret Key: 用于对请求进行签名和校验的密钥，须保密，可通过 AWS SDK 配置。

- Endpoint: Proxy 的**接入地址**，可通过 AWS SDK 配置。

- 用户: 一个拥有 Access Key ID 和 Secret Key 的实体，通常是**个人**或者**一个服务**。

- 用户组: 一组用户。目前约定一个业务团队的服务归为一组。

- 策略: 一组关于访问控制的规则，内含"权限"，详见[设计](/docs/developer/design/rule/model/object#策略)。关联至用户/组/角色。

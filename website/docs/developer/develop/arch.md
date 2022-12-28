---
sidebar_position: 3
---

# 程序架构

本章介绍 PIAM 及 Proxy Layer 体系中各组件内部的程序架构，宏观(业务)架构请参看 [方案设计](/docs/developer/design/intro#宏观架构)。

## Proxy

对应方案设计中的 [Compliant Proxy](/docs/developer/design/intro#概念定义) 组件。

1. Proxy 在代码组织上采用了微核架构，无插件模块 (Plugin) 时，其自身**不感知任何业务**，是一个没有任何实际作用的 L7(目前) 代理。
2. 如插入 IAM(piam-proxy) 模块，则其成为一个具有基于属性的**通用访问管控功能**的代理。
3. IAM(piam-proxy) 模块本身也是 **pluggable** 的设计。
4. 对 IAM(piam-proxy) 插入 **S3协议解析器** 和 **对象存储策略匹配器**，则 Proxy 成为 **S3-like** 服务的安全访问层。
5. 对 IAM(piam-proxy) 插入 **AWS 签名模块**，则 Proxy 能够**兼容 AWS** 的账号体系及鉴权体系。

![微核架构](/img/microkernel-architecture.png)

:::tip 插件模块 (Plugin)

在 Rust 语境中，Plugin 表达为一个 [feature](https://doc.rust-lang.org/cargo/reference/features.html)，通过编译选项决定是否编译链接进入二进制产物。

:::

## Manager

对应方案设计中的 [Rule Provider / Manager](/docs/developer/design/intro#概念定义) 组件。

...

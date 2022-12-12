---
sidebar_position: 2
---

# Compliant Proxy

Compliant Proxy 被设计为对 [RS](/docs/developer/design/model/model/common) 只读的无状态 Consumer，以支持水平扩展。

## 模块设计

由于文档型数据库并没有一个比较标准化的协议，对 dynamodb 和 mongo 的协议解析后需要各写一个薄的适配层，将请求结构转到自定义的标准化 AST，
再基于标准 AST 实现 Policy 表达的语义，同时对各 IAM 实体进行搜索与匹配，最后 apply effects。涉及关键模块如下：
- dynamo/mongo protocol parser module as plugin
- dynamo/mongo to _document_ adapter module as plugin
- iam module as plugin
  - iam container sub module
  - cloud signature sub module
  - document policy matcher sub module
  - policy effect executor sub module
  
由于 AWS S3 协议是对象存储行业的事实标准，相比于文档型数据库少了一个薄的适配层。

## 分布式状态

如限流场景，CP 从 [RM](/docs/developer/design/components/rule-provider) 拉取当前 CP 节点数，并自行计算单节点阈值。

## 持久化

可选用 DynamoDB/MongoDB 作为安全事件(结构化日志)的存储。

## 通信

CP 之间本身不通信，除了存储资源本身，CP 唯一必须通信的对象是 RM。

目前通信机制设计为 CP 向 RM 发起 HTTP 请求以获得 RS。后续演进时可考虑设计实时性更好的通信方式。 

心跳机制，周期性向 RM 上报自身的健康状态，这样系统对于 CP 节点数的感知不依赖 k8s。

## 部署

- CP 部署在离数据源近的点，减小数据读写链路上这个多出来的 RTT。

[//]: # (// todo)

[//]: # (- connect type)

[//]: # (  - http)

[//]: # (  - tcp)

[//]: # (- 要考虑多CP扩容的时候，流控的阈值怎么实时更新)

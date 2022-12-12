---
sidebar_position: 4
---

# Rule Factory

Rule Factory (RF) 是一种实践 DataOps & MLOps 的构想，它的关注点是对数据的持续发现、转化与交付。

这里提出一些 RF 试图提供的特性与用例：

- 持续地收集由 [CP](/docs/developer/design/components/compliant-proxy) 产生的事件，基于 ML 识别潜在安全风险。
- 持续地收集由 CP 产生的事件，基于(半)监督学习输出符合智慧芽安全场景所期望的 [RS](/docs/developer/design/model/model/common)。
- 用户输入自然语言，RF 通过语义识别来生成或变更 RS。如钉钉告警群对某条告警回复"关闭这类安全事件的通知"。
- ...

:::caution 过度设计

为避免引入过多复杂性，RF 组件不在早期架构中出现。

:::
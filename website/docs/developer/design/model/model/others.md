---
sidebar_position: 9
---

# 其他数据模型

对于其他资源模型的访问管控，此方案预留了扩展点。

扩展方式为：

1. 定义该数据模型的[策略](/docs/developer/design/model/model/common#策略-policy)与[输入](/docs/developer/design/model/model/common#资源客体--操作-input)。
2. 按照[模块设计样例](/docs/developer/design/components/compliant-proxy#模块设计)的描述实现该数据模型对应的解析、标准化与策略匹配模块。

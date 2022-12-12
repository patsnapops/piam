---
sidebar_position: 2
---

# 文档型

这里描述文档型数据库的策略结构样例和输入样例，主要针对 DynamoDB 和 MongoDB。

## 策略

```yaml title="policy_document.yaml"
version: 1
kind: Document
table:
  name:
    in: [name1, name2]
      field: # optional
        name:
          in: [name3, name4]
```

:::tip 维度

以上列举了最基础的名称维度，后续演进过程允许加入表创建时间、文件类型的价值等多重过滤维度。

:::

## 输入

计划支持的 [操作类型](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Operations_Amazon_DynamoDB.html) 优先级如下：

- DeleteDoc
- GetDoc
- PutDoc
- UpdateDoc
- Scan
- BatchGetDoc
- BatchWriteDoc
- ...

## 指纹方案

针对特定[情况](/docs/developer/design/model/model/common#情况-condition)的大规模文档导出行为，在文档集中掺入指纹文档。

## 脱敏方案

针对特定情况特定文档的读取行为，如非用户业务相关的[主体](/docs/developer/design/model/model/common#主体-principal)读取用户表，
通过[声明](/docs/developer/design/model/model/common#声明-statement)对特定字段脱敏处理。

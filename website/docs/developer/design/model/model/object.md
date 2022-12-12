---
sidebar_position: 1
---

# 对象存储型

这里描述对象存储如腾讯云 COS、AWS S3 的策略结构样例和输入样例。

## 策略

```yaml title="policy_object_storage.yaml"
version: 1
kind: ObjectStorage
bucket:
  name:
    in: [name1, name2]
      object: # optional
        path:
          startWith: [dir_name1, dir_name2]
```

:::tip 维度

以上列举了最基础的名称维度，后续演进过程允许加入桶创建时间、字段价值等多重过滤维度。

:::

## 输入

计划支持的 [操作类型](https://docs.aws.amazon.com/AmazonS3/latest/API/API_Operations_Amazon_Simple_Storage_Service.html) 优先级如下：

- DeleteObject
- GetObject
- PutObject
- ...

## 指纹方案

针对特定[情况](/docs/developer/design/model/model/common#情况-condition)的 GetObject，对文件插入二进制数据。
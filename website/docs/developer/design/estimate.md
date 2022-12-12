---
sidebar_position: 5
---

# 体量概算

## 空间复杂度

若以 AWS IAM 单账户各实体的配额上限为基准，在无空间浪费的情况下系统至多需约 1.16 GB 的空间来存储 [**Rule Spec**](/docs/category/关键实体)。

![](/img/iam_size.png)

在工程实现上，如选型采用基于对象模型与垃圾收集机制的语言运行时，后期对于 in-memory 的 RS 数据结构可做一定的优化设计来抵消 bloating。

:::note 云厂对于空间复杂度的限制

[资料1](https://cloud.tencent.com/document/product/598/10609)

[资料2](https://aws.amazon.com/premiumsupport/knowledge-center/iam-increase-policy-size/)

[资料3](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entities)

:::

## 时间复杂度

...

[//]: # (定义：)

[//]: # ()
[//]: # (Mc 代表运行时命中的 condition 个数)

[//]: # (Me 代表定义的 effect 个数)

[//]: # (对于一次数据操作的管控，最差时间复杂度为 O&#40; &#40;Np + Nc + Ndo + Nda + Ne&#41; * Mc * Me &#41;。)

[//]: # ()
[//]: # (所以定义 RS 时需要注意 condition 的数量不能过多，对于不能异步执行的 effect 类型 &#40;modify&#41; 需谨慎添加。)

[//]: # ()
[//]: # (空间复杂度)

[//]: # (对于全量 RS 的存储，最差空间复杂度为 S&#40; Np * Nc * Ndo * Nda * Ne &#41;。)

[//]: # ()
[//]: # (如 Np 数量为 1000，Nc 数量为 5，Ndo 数量为 1000，Nda 数量为 20，Ne 数量为 5，平均每个 N 20 字节，所需存储空间为 500 M * 20 B = 10 GB， 所以对 in-memory 的 RS 数据结构需考虑做一定的优化设计。)


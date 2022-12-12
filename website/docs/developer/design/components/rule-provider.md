---
sidebar_position: 3
---

# Rule Provider / Manager

Rule Provider / Manager 是非计算密集的组件，它将 [RS](/docs/developer/design/model/model/common) 的定义与管理从 [CP](/docs/developer/design/components/compliant-proxy) 中解耦出来，
使得 CP 只需关心关于 RS 的计算。

## API 设计

- 提供查询什么密钥对应什么 RS 的接口，供云管等自助平台集成
- ...

## 模块设计

- RS api server
- RS analyzer
- ...

### RS Api Server

RS Api Server 的目标是提供各种操作 RS 相关的接口供 CP 及其他系统调用。

### RS Analyzer

RS Analyzer 通过静态分析检测出 RS 策略树中的策略冲突与冗余策略，其意义在于：

- 减少繁琐的人工 RS 校验工作，保证系统的正确性
- <details>
  <summary>减少 CP 的代码复杂度与运行时开销</summary>
  
  :::caution 无法通过静态分析来控制的复杂度
  在运行时，一次数据操作可能会同时符合多种[情况](/docs/rule/model/common.md#情况)。根据最小权限原则，[CP](/docs/components/compliant-proxy.md)
  必须深度遍历所有子节点，找到最严格的约束来执行，并且对观测和修改去重，这可能使得 CP 无法保持 _Predictable Performance_。
  :::
    
  </details>


:::note 参考

[代码生成与索引方式](https://analytics.zhihuiya.com/search/result/tablelist/1?sort=sdesc&limit=20&q=abac%20%E5%86%B2%E7%AA%81%E6%A3%80%E6%B5%8B&_type=query)

[屏蔽码方式](https://www.jsjkx.com/CN/10.11896/j.issn.1002-137X.2018.02.034)

:::

## 持久化

使用 DynamoDB/MongoDB 作为安全层规则的存储。

## 自身安全

- Meta key for CP to get real secret keys
- 对 secret keys 的储存加密
- 做基于 IP 的访问限制
- 在文件系统层或者硬件层做存储加密
- ...

## 部署

RM 不像 CP 需要高吞吐，可进行少量副本部署提高可用性。

### 跨可用区同步

每个可用区独立部署 RM。

对一些 RS，有跨 Region/AZ 同步的需求。由于 RM 自身无状态，从数据库读取对应的 RS 写入目标区域库达成最终一致即可。

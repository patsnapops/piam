---
sidebar_position: 1
---

# 方案概览

这是一份关于安全访问 Patsnap 资源的设计草案。

:::tip 浏览手册

点击**左侧**列表进行章节跳转，**右侧**列表进行页内跳转。点击**绿色**超链接文本跳转到定义。

:::

## 背景

目前公司技术栈依赖亚马逊的 S3、DynamoDB、SQS 等线上服务，且**身份认证和访问管控体系与其深度绑定**，在迁云、跨云的战略背景下，提出了新的挑战：

- 如何避免**密钥外流**导致的数据泄露？
- 如何管理各云厂的 IAM，避免策略与使用方式**不一致性**带来的**成本**？
- 如何让**跨云异构**数据资源的访问管控对上层做到无感、透明？
 
此方案在技术落地层面优先考虑对[存储资源](#资源类型)进行数据级访问管控的实现，即部门 OKR 中的控制权限最小化、 识别异常操作风险。
**例如能够实时阻断对 S3 对象进行大量删除的行为。**

## 设计目标

PIAM 试图为 Patsnap 应用对资源的访问提供**统一的安全模型**，这里定义一些它能够解决的问题和特性：

- 对象存储、数据库**资源**的字段级访问管控。
- **多维**访问管控。如基于数据操作类型的阻断、基于数据流向的指纹等。
- **非侵入式**，不触发阻断的情况下，上层无感知。
- 能够对安全事件进行**溯源**。

该方案的设计目标于 2022 Q3 进行了调整，增加了对消息队列进行支持的需求。并且重点优先放在对云商密钥的回收上。

## 边界定义

这里定义一些问题域的边界。

以下问题目前**不在**此方案试图解决的范围内：

- ~~非存储层的访问管控。如针对计算引擎、消息队列等中间件操作的管控。~~
- OS 层、硬件层的存储资源访问管控。如针对文件系统、块设备操作的管控。
- 通过此系统正常拿到数据后的泄露。如业务服务通过预定义的权限在已拿到数据的情况下，因业务系统自身漏洞导致的数据流出。
- 非安全层面的特性。如针对存储层的负载均衡、缓存。
- 此系统自身依赖的存储的访问管控。如此系统对存储其自身信息的数据库进行查询，无需经过其自身的安全机制审查。

## 资源类型

<table>
   <tr>
      <th>#</th>
      <th>Unstructured Data</th>
      <th colspan="6" align="center" >Structured / Semi Structured Data</th>
   </tr>
   <tr>
      <th>Data Model</th>
      <td>Object Storage</td>
      <td>Document</td>
      <td>Search Engine</td>
      <td>Relational</td>
      <td>Key Value</td>
      <td>Time Series</td>
      <td>Graph</td>
   </tr>
   <tr>
      <td>1</td>
      <td>aws s3**</td>
      <td>dynamo**</td>
      <td>solr</td>
      <td>tidb (mysql)</td>
      <td>redis</td>
      <td>prometheus-like</td>
      <td>neo4j</td>
   </tr>
   <tr>
      <td>2</td>
      <td>tencent cos**</td>
      <td>mongo*</td>
      <td>elasticsearch</td>
      <td>postgres</td>
      <td>tikv</td>
      <td>others</td>
      <td>neptune</td>
   </tr>
</table>

:::tip 模块化

带 * 为优先支持的存储资源，其余类型目前不纳入此方案的解决范围，但文档中讨论了[可扩展的方式](/docs/developer/design/model/model/others)。
其中各存储类型大致按目前智慧芽业务对其依赖度排序。

:::

## 概念定义

这里定义一些问题域相关的概念。

要实现对于资源操作的观测与管控，势必要有某种形式的规则，在数据的读写链路中加入一定的处理。

处理的结果可能是允许，也可能是不允许，也可能是部分允许，比如限流、脱敏等。

在此文档中：

- 称描述这些规则的对象为 [**Rule Spec**](/docs/category/关键实体) (RS)
- 称提供与管理这些规则的对象为 [**Rule Provider / Manager**](/docs/developer/design/components/rule-provider) (RM)
- 称对数据流做处理来满足这些规则的对象为 [**Compliant Proxy**](/docs/developer/design/components/compliant-proxy) (CP)

## 宏观架构

整个存储安全控制层工作在实际的存储层之上，业务服务层&数据生产层(data、sa)之下。

![Arch](/img/arch.svg)

整体 3 层：

- 智慧芽业务/数据生产层
- Patsnap IAM 层
- 各云厂服务层(或自建的 DB/MQ/ObjectStorage/Internal Resource)

绿色虚线框即 Patsnap IAM 层，内部主要有 2 种角色(组件)：

- Manager 负责管理用户身份与访问策略。
- Compliant Proxy 负责接管流量进行访问控制，会与 Manager 通信获取必要信息。

业务/数据层使用 Patsnap IAM 层提供的 Credential (ak/sk, username/pwd, etc.) 来访问各云厂服务。代码无需改造，仅替换 Endpoint 配置。

以上为 PIAM 顶层概览，了解更下层的抽象请继续阅读。

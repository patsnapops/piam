---
sidebar_position: 0
---

# 通用部分

**PIAM 是对各云厂 IAM 的一层抽象。**

[Rule Spec](/docs/developer/design/intro#概念定义) 是 PIAM ABAC 模型的 specification，它描述了主体在何种情况下对何种资源客体进行何种操作会受到何种效果的影响。

## 访问管控的效果 Effect

针对资源级别的访问管控往往采用布尔逻辑来描述匹配到规则后产生的影响效果 (Effect)，即允许或拒绝。

:::note 各云厂访问管控的效果

[AWS](https://docs.aws.amazon.com/zh_cn/IAM/latest/UserGuide/access_policies.html#access_policies-root)，
[腾讯云](https://cloud.tencent.com/document/product/598/10604#.E8.AF.AD.E6.B3.95.E6.8F.8F.E8.BF.B0)，
[阿里云](https://help.aliyun.com/document_detail/125505.html#section-i41-bus-mq6)

:::

而智慧芽安全场景则要求对数据级别的访问提供更多维的访问管控效果，如：

- 观测 - 日志、告警 ...
- 约束 - 阻断、限流 ...
- 修改 - 指纹、脱敏 ...

[//]: # (observation restriction modification 有个很好的应用场景比如ci环境开发自己拉数据来测，对于用户密码这类敏感字段，可以转成*号来代替)

<details>
<summary>定义</summary>

```yaml title="effects.yaml"
version: 1
kind: effects
types: 
  observe:
    log:
      dest:
        file:
        mq:
        mongodb:
        dingTalk:
    metric:
      prometheus:
  restrict:
    deny: 
    ratelimit: 
      period:
      actionCount:
  modify:
    fingerprint:
    redaction: 
      policy:

```

</details>

## 访问管控的上下文

### 主体 Principal

在 ABAC 模型的应用中，主体仍是最常见的属性，通常是**用户 (User)**或者角色 (Role)。用户可以关联多个用户组。

<details>
<summary>定义</summary>

```yaml title="principal.yaml"
version: 1
kind: user
person:
  id: 
  name: 
  teamId: 
  accessKey: PERSXXX # prefix for easy recognition
  secretKey: 
```

</details>

### 资源客体 & 操作 Input

Input 需要关心资源访问所使用的协议。以[对象存储](/docs/developer/design/model/model/object)为例，其资源客体即为储存桶及其储存的文件对象，操作为对其自身及其元数据各种类型的读写。

### 策略 Policy

策略包含 2 部分

- 对各资源访问协议都**通用**的情况
- 对各资源访问协议**不通用**的声明

#### 情况 Condition

情况描述了访问资源需满足的条件，如特定 IP 范围的访问管控。

<details>
<summary>定义</summary>

```yaml title="condition.yaml"
version: 1
kind: condition
types:
  regionIn:
  ip: # CIDR
  timeBetween:
  MFAPresent:

```
</details>

#### 声明 Statement

Statement 需要关心资源本身的数据模型，用于描述 input 与 effect**s** 之间的匹配关系。

## Rule Spec 的动态性

以下列出了 RS 的动态性等级：

1. 硬编码在程序中
2. 作为配置在 [Compliant Proxy](/docs/developer/design/components/compliant-proxy) 启动时加载
3. 以[服务](/docs/developer/design/components/rule-provider)的方式提供 RS，来支持实时性的变更
4. 通过[计算](/docs/developer/design/components/factory)而不是通过人工预定义来产生或变更 RS

## Rule Spec 的可变性

策略被设计为不可变的 (_Immutable_)，对一个策略进行任何修改都会创建一个新的 Policy 并分配一个 UUID。

<details>
<summary>Why?</summary>

想象如果 RS 是可变的：

1. 一次匹配到某个 Policy 的安全事件被记录下来，该记录中包含安全事件关联的 Policy 信息 (RS UUID_1)。
2. 这个 Policy 被修改了。
3. 另一次匹配到这个 Policy 的安全事件被记录下来，该记录中包含安全事件关联的 Policy 信息 (RS UUID_1)。
4. 对安全事件审计时，不一致性发生了 - 审计者发现对于同一个 Policy，所对应的安全事件却完全不一样。
5. 并且 1. 中旧 RS 的持久化状态丢失，在审计追溯安全问题、日常排查主体操作受限原因时，可变性将使这个过程变的非常困难。

</details>

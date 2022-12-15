---
sidebar_position: 4
---

# 错误处理

本项目关于错误处理的约定。

## 前提

需先阅读 [前提准备](/docs/developer/develop/intro) 等章节。

## 三原则

1. 除测试代码，不使用 panic、unwrap，而应返回 `Result` 类型。如必须使用，需**添加注释**说明原因。
2. 非紧急需求交付，不使用 expect，而应返回 `Result` 类型。必须使用 expect 时，需要为 `msg` 参数提供**上下文说明**。
3. 在**合适**的层次处理 error 的 `Result`，当前层无法处理时携带上下文并使用 `?` 向上返回。 

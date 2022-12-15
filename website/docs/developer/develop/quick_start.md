---
sidebar_position: 4
---

# 调试流程

## 前提

1. 需先阅读或了解方案设计、程序架构等章节。
2. 打通网络，使 https://crates.io/ 和 Github 能正常访问。

## 以 PIAM S3 Proxy 的集成测试流程为例

### 1. 准备调测数据

打开终端 1

1. 编写 `crates/piam-manager/tests/iam.rs` 代码准备调测数据
2. `cd ./crates/piam-manager/`
3. `cargo test --all-features -- "write_all" --show-output`

### 2. 启动 PIAM Manager

打开终端 1

1. `cd ./crates/piam-manager/`
2. `cargo watch -x 'run --bin piam-manager -- dev' -d1`

### 3. 启动 S3 Proxy

打开终端 2

1. `cd ./crates/s3-proxy/`
2. `cargo watch -x 'run --bin s3-proxy --all-features -- dev' -d1`
3. S3 Proxy 将被编译、启动，并从 PIAM Manager 预加载调测数据并初始化
4. 对相关代码进行调整
5. S3 Proxy 将自动编译、重启、初始化
6. 观察日志输出，是否达到迭代目标
7. 对相关代码进行调整
8. S3 Proxy 将自动编译、重启、初始化
9. ...

## 以 PIAM S3 Proxy 的单元测试流程为例

...

---
sidebar_position: 1
---

# 注释&文档&格式

## 注释 & 文档

https://course.rs/basic/comment.html

https://www.docusaurus.cn/docs

## 代码格式 (Formatting)

### Rustfmt config

```toml title="http://git.patsnap.com/devops/security/piam/blob/dev/rustfmt.toml"
edition = "2021"
reorder_imports = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_modules = true
merge_derives = true
use_field_init_shorthand = true
format_macro_matchers = true
format_macro_bodies = true
```

### 提交前执行

```bash
cargo +nightly fmt --all
```
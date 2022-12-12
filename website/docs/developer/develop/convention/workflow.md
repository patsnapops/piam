---
sidebar_position: 3
---

# 分支工作流

一种适应较快交付节奏的策略。

## 原则

只有正确和稳定的代码才能被并入 master 分支。

## 三类分支

- master (长期分支)
- dev (长期分支)
- 短期分支

## 流程

1. 从 dev 分支检出短期分支开发新特性、修复问题，完成后并入 dev 分支。
2. 如需 review，在并入 dev 时实施，并入后发布运行检验。
3. dev 分支运行一段时间无问题后并入 master 分支，并在 master 上 tag 版本号。

参考：

[https://www.git-tower.com/learn/git/ebook/cn/command-line/branching-merging/branching-workflows](https://www.git-tower.com/learn/git/ebook/cn/command-line/branching-merging/branching-workflows)
---
sidebar_position: 2
---

# Commit message

每次提交，Commit message 都包括三个部分：Header，Body 和 Footer。

```
<type>(<scope>): <subject>
// 空一行
<body>
// 空一行
<footer>
```

其中，Header 是必需的，Body 和 Footer 可以省略。推荐写完将 subject 内容复制进[谷歌翻译](https://translate.google.com/)里并修改，使语义准确、明确。

## Header

Header部分只有一行，包括三个字段：`type`（必需）、`scope`（可选）和`subject`（必需）

### type

`type` 代表某次提交的类型，比如是修复一个bug还是增加一个新的feature。所有的type类型如下：

- feat: 新增feature
- fix: 修复bug
- refactor: 代码重构，没有加新功能或者修复bug
- perf: 优化相关，比如提升性能、体验
- docs: 仅仅修改了文档，比如README, CHANGELOG, CONTRIBUTE等等
- style: 仅仅修改了空格、格式缩进、都好等等，不改变代码逻辑
- test: 测试用例，包括单元测试、集成测试等
- ci: 改变构建流程 CI/CD 相关
- chore: 修改依赖库版本等杂项
- revert: 回滚到上一个版本

如果`type`为`feat`和`fix`，则该 commit 必须出现在 Change log 之中。

### scope

`scope`用于说明 commit 影响的范围，比如数据层、控制层、视图层等等，视项目不同而不同。

### subject

`subject`是 commit 目的的简短描述，不超过50个字符。

> 以动词开头，使用第一人称现在时，比如change，而不是changed或changes第一个字母小写结尾不加句号（.）
>

### mark

推荐使用在`type`、`scope` 后添加感叹号表示与不兼容的更新(breaking change)，即公开给用户的接口做了变更。

```bash
feat!: send an email to the customer when a product is shipped
```

```bash
feat(api)!: send an email to the customer when a product is shipped
```

## Body

Body 部分是对本次 commit 的详细描述，可以分成多行。下面是一个范例。

> More detailed explanatory text, if necessary.  Wrap it to
about 72 characters or so.

Further paragraphs come after blank lines.

- Bullet points are okay, too
- Use a hanging indent
>

有两个注意点。

（1）使用第一人称现在时，比如使用`change`而不是`changed`或`changes`。

（2）应该说明代码变动的动机，以及与以前行为的对比。

## Footer

Footer 部分只用于两种情况。

**（1）不兼容变动**

如果当前代码与上一个版本不兼容，则 Footer 部分以`BREAKING CHANGE`开头，后面是对变动的描述、以及变动理由和迁移方法。

> BREAKING CHANGE: isolate scope bindings definition has changed.

    To migrate the code follow the example below:

    Before:

    scope: {
      myAttr: 'attribute',
    }

    After:

    scope: {
      myAttr: '@',
    }

    The removed `inject` wasn't generaly useful for directives so there should be no code using it.
>

**（2）关闭 Issue**

如果当前 commit 针对某个issue，那么可以在 Footer 部分关闭这个 issue 。

> Closes #234
>

也可以一次关闭多个 issue 。

> Closes #123, #245, #992
>

参考：

[https://www.conventionalcommits.org/en/v1.0.0/](https://www.conventionalcommits.org/en/v1.0.0/)

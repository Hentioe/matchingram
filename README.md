# MatchinGram

一个通用的高性能的匹配引擎，利用精心设计的可读规则自由匹配各式各样的 Telegram 消息！

## 介绍

这是一个以高自由度、极限速度和开销恒定为目的开源库。它参考了 Cloudflare 的防火墙规则的设计，以 Telegram 消息为目标进行了定制支持。

机器人的很多功能都依赖对特定消息的匹配，通过本库可以更轻松优雅的实现这一部分。更重要的是，对匹配逻辑的构建可以交给用户！一些常见的例子：

#### 欢迎消息机器人

匹配包含新成员的消息。通过代码组织逻辑，它大概是这样子的：

```javascript
if (message.new_chat_members != null && message.new_chat_members.length > 0) {
  // 发送欢迎消息
}
```

使用匹配引擎，它将是这样一条简单的规则：

```
(message.new_chat_members)
```

这条规则交给用户编写，并让用户选择（命中后）与之搭配的「执行动作」即可。对于欢迎消息，执行动作就是发送指定消息内容。

#### 关键字回复机器人

匹配包含指定关键字的消息文本。通过代码组织逻辑，它大概是这个样子的：

```javascript
if (
  message.text != null &&
  message.text.contains("关键字1") &&
  message.text.contains("关键字2")
) {
  // 发送对应的回复
}
```

使用匹配引擎，它将是这样一条规则：

```
(message.text contains_all {"关键字1" "关键字2"})
```

这个简单的需求无法体现规则的全部。规则对于文本的匹配支持是放在首位的，它能做到的比这个强得多得多。

## 规则设计

规则可视作多个“条件组”的集合。一般条件由“字段” + “运算符” + “值” 构成，条件可具备 `and` 或 `or` 关系，不能嵌套。

- 在一般条件的构成基础上，前置 `not` 可表示取反。
- 字段由多个单词组合而成，通过点（`.`）连接。运算符则使用 snake_case 的风格命名。
- 单值分为字符串和数字。前者使用双引号（`""`）包裹，后者不需要。
- 多值用大括号（`{}`）包裹多个单值，并以空格间隔。多值即「值的列表」。
- 相邻的具有 `and` 关系的条件在同一个括号中，但相邻的 `or` 关系的条件之间彼此独立。
- 不具有运算符和值的条件直接使用字段构成，前置 `not` 亦可取反。例如：`(message.from.is_bot)` 以及前文中的第一个案例。

一个五脏俱全的例子：

```
(message.text.size gt 120 and message.from.is_bot) or
(not message.from.is_bot and message.from.fullname contains_one {"bot", "机器人"}) or
(
  not message.from.id in {10086 10010} and
  message.text contains_one {"移动", "联通"} and
  message.text contains_one {"我是", "客服"}
)
```

这条规则将匹配：由机器人发送的文本内容超过 120 个字的消息。或者，名字中包含“机器人”或 "bot"（既冒充机器人）的用户发送的消息。或者，用户 ID 不是
10086 或 10010 却自称移动或联通的客服的消息！

没错，这条规则的匹配逻辑可能没有什么实际意义。但想用代码表达这样奇葩的逻辑却不容易，更难以提供灵活的动态设置让普通用户也可以容易的组合出来。

匹配规则的高可读性和高自由度，让这成为现实。这便意义所在。

你可能注意到，这条规则被有意的换行和格式化了。没错，因为规则中的分隔符可以是不受数量限制的空白字符。也许，你还会考虑，这条规则是不是有点长了？

不，它太小了。即使规则文本达到 MB 的大小，引擎也能在毫秒级的速度内完成解析（甚至结束匹配）。

## 条件文档

本章节将会介绍字段和运算符以及值的概念，并且罗列所有的字段和运算符以及相对应的说明。

_待更新……_

## 实际案例

本章节将会展示一些常见需求下的规则例子，作为参考方便用户学习。

_待更新……_

## 值表达式

本章节将会介绍一个重要的后续计划，它可以让文本内容匹配更加精准和强大。

_待更新……_

## 性能优化

本章节将会介绍作为开发者，如何使用本库提供的优化相关函数。通过预编译和规则优化，让匹配速度达到极限。

_待更新……_

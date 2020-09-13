# MatchinGram

一个通用的高性能的匹配引擎，利用精心设计的可读规则自由匹配各式各样的 Telegram 消息！

## 介绍

这是一个以高自由度、极限速度和恒定开销为目标的开源库。它参考了 Cloudflare 的防火墙规则的设计，以 Telegram 消息为目标进行了定制支持。

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

匹配包含指定关键字的文本消息。通过代码组织逻辑，它大概是这个样子的：

```javascript
if (
  message.text != null &&
  message.text.contains("关键字1") &&
  message.text.contains("关键字2")
) {
  // 发送相应的回复
}
```

使用匹配引擎，它将是这样一条规则：

```
(message.text all {"关键字1" "关键字2"})
```

这个简单的需求无法体现规则的全部。规则对于文本的匹配支持是放在首位的，它能做到的比这个强得多得多。

#### 服务消息清理机器人

匹配各种服务消息。通过代码组织逻辑，它大概是这个样子的：

```javascript
if (
  message.new_chat_members ||
  message.new_chat_title ||
  message.new_chat_photo ||
  message.pinned_message // 等等 ……
) {
  // 执行删除动作
}
```

使用匹配引擎，它将是这样一条规则：

```
(message.is_service_message)
```

_此处不再详细举例，更多例子可参考[实际案例](#实际案例)。_

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
(not message.from.is_bot and message.from.full_name any {"bot", "机器人"}) or
(
  not message.from.id in {10086 10010} and
  message.text any {"移动", "联通"} and
  message.text any {"我是", "客服"}
)
```

这条规则将匹配：由机器人发送的文本内容超过 120 个字的消息。或者，名字中包含“机器人”或 "bot" 的用户发送的消息。或者，用户 ID 不是
10086 或 10010 却自称移动或联通的客服的消息！

没错，这条规则的匹配逻辑可能没有什么实际意义。但想用代码表达这样奇葩的逻辑却不容易，更难以提供灵活的动态设置让普通用户也可以容易的组合出来。

匹配规则的高可读性和高自由度，让这成为现实。这便意义所在。

你可能注意到，这条规则被有意的换行和格式化了。是的，为了可读性允许规则使用任意数量的空格或换行作为间隔符。也许，你还会考虑，这条规则是不是有点长了？

不，它太小了。即使规则文本达到 MB 的大小，引擎也能在毫秒级的速度内完成解析（甚至结束匹配）。

## 基准测试

以下测试结果可通过拉取源代码执行 `cargo bench` 获得。

| 函数           |        参数条件         |   结果    | 备注                                       |
| :------------- | :---------------------: | :-------: | ------------------------------------------ |
| `rule_match`   |     `regular-rule`      | 1.5388 us | 标准长度规则匹配                           |
| `rule_match`   | `regular-negative-rule` | 1.6039 us | 标准长度规则取反匹配                       |
| `rule_match`   |       `long-rule`       | 3.2567 us | 长规则匹配                                 |
| `rule_match`   |      `longer-rule`      | 3.3729 us | 更长的规则匹配                             |
| `compile_rule` |       `1mb-rule`        | 11.929 ms | 1MB 大小的规则编译（解析）                 |
| `rule_match`   |    `worst-1mb-rule`     | 12.428 ms | 1MB 大小的规则匹配（匹配到末尾的最糟情况） |

如上所见，正常或正常稍长的规则都能在纳秒级的速度内完成匹配。即使规则文本数据有 1MB 大小（可能有数万行）也能在 10 毫秒上下解析完成或匹配结束。

规则的最终的目的和正则表达式有部分重叠，但正则表达式是开销不恒定的东西。在几乎任何系统的设计上都不建议允许用户直接输入的正则表达式，因为攻击者能利用病态正则（专门写出的速度特别慢的表达式）轻易的将系统资源耗光。哪怕是 Cloudflare 也曾因此出过事故（[详细](https://blog.cloudflare.com/details-of-the-cloudflare-outage-on-july-2-2019/)）。并且正则也做不到对消息进行条件化的复杂匹配（因为消息是结构化数据），它适合对单个关键字实施更精准的匹配（但如上原因，它不应该被支持）。

匹配引擎恒定的开销意味着无论用户输入怎样的规则，都不会影响系统的稳定性。随着规则数量的与日俱增也只会令系统开销呈稳定的线性增长。

## 条件文档

在阅读本章之前你必须阅读[规则设计](#规则设计)，了解规则的语法以及条件的构成。

不过，规则的核心仅仅只是“条件”，规则的语法只是条件之间的关系的表达方式。问题语法会被精准的报错，因为匹配引擎对规则表达式的解析会经历一个完整的词法到文法分析的过程（它们是编译流程的一部分）。

下面是对条件构成部分的详细解释。

### 字段

条件中不可或缺的一部分，它表示“匹配的目标内容”。

例如匹配消息文本（`message.text`）或匹配消息来源（`message.from`）。不过消息来源自身没有可匹配的内容，应该访问其具体字段，例如来源用户是否是机器人（`message.from.is_bot`）。

**注意**：字段和 Telegram 的消息结构并不全部一一对应，有一部分是本项目针对性新增的。具体存在哪些字段，以及字段的含义请参照[字段说明](#字段说明)。

### 运算符

条件中可选的一部分，它表示“执行匹配的方法”。

例如等于（`eq`）、包含列表中的任意一个（`any`）、大于（`gt`）或属于列表之中（`in`）。

这些不同的运算符告诉了引擎如何去匹配目标内容。而运算符是否支持是由字段决定的，例如所有文本类型的目标内容都不支持大于、小于这类数字比较运算符。

**注意**：不需要运算符的字段往往是布尔类型的，也有可能是对字段的非空判断（内容不为空）。

### 值

条件中可选的一部分，它表示“运算符的参数”。

例如单值字符串（`"小黄鸡"`）或单值数字（`12345678`）或字符串列表（`{"小明" "小红" "小象"}`）或数字列表（`{10086 10010}`）。

其中数字的取值范围是 64 位带符号整型或浮点型，可涵盖 Telegram 的所有 ID 范围。

值的类型是由运算符决定的，例如 `eq` 运算符只是内容比较是否相等，不需要列表类型的值。

**注意**：不需要运算符的字段也不需要值。

### 支持详情

以下表格中勾选的运算符表示该字段支持，未勾选表示不支持。

| ↓ 字段/运算符 →                   | `eq` | `gt` | `ge` | `le` | `in` | `any` | `all` | `hd` |
| :-------------------------------- | :--: | :--: | :--: | :--: | :--: | :---: | :---: | :--: |
| `message.from.id`                 |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.from.is_bot`             |      |      |      |      |      |       |       |      |
| `message.from.first_name`         |  ✓   |      |      |      |  ✓   |   ✓   |   ✓   |  ✓   |
| `message.from.full_name`          |  ✓   |      |      |      |  ✓   |   ✓   |   ✓   |  ✓   |
| `message.from.language_code`      |  ✓   |      |      |      |  ✓   |       |       |  ✓   |
| `message.forward_from_chat`       |      |      |      |      |      |       |       |      |
| `message.forward_from_chat.id`    |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.forward_from_chat.type`  |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.forward_from_chat.title` |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.reply_to_message`        |      |      |      |      |      |       |       |      |
| `message.text`                    |  ✓   |      |      |      |  ✓   |   ✓   |   ✓   |  ✓   |
| `message.text.size`               |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.animation`               |      |      |      |      |      |       |       |      |
| `message.animation.duration`      |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.animation.file_name`     |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.animation.mime_type`     |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.animation.file_size`     |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.audio`                   |      |      |      |      |      |       |       |      |
| `message.audio.duration`          |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.audio.performer`         |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.audio.mime_type`         |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.audio.file_size`         |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.document`                |      |      |      |      |      |       |       |      |
| `message.document.file_name`      |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.document.mime_type`      |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.document.file_size`      |  ✓   |  ✓   |  ✓   |      |      |       |       |      |
| `message.photo`                   |      |      |      |      |      |       |       |      |
| `message.sticker`                 |      |      |      |      |      |       |       |      |
| `message.sticker.is_animated`     |      |      |      |      |      |       |       |      |
| `message.sticker.emoji`           |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.sticker.set_name`        |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.video`                   |      |      |      |      |      |       |       |      |
| `message.video.duration`          |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.video.mime_type`         |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.video.file_size`         |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.voice`                   |      |      |      |      |      |       |       |      |
| `message.voice.duration`          |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.voice.mime_type`         |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.voice.file_size`         |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.caption`                 |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.caption.size`            |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.dice`                    |      |      |      |      |      |       |       |      |
| `message.dice.emoji`              |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.poll`                    |      |      |      |      |      |       |       |      |
| `message.poll.type`               |  ✓   |      |      |      |  ✓   |       |       |      |
| `message.vence`                   |      |      |      |      |      |       |       |      |
| `message.vence.title`             |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.vence.address`           |  ✓   |      |      |      |      |   ✓   |   ✓   |  ✓   |
| `message.location`                |      |      |      |      |      |       |       |      |
| `message.location.longitude`      |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.location.latitude`       |  ✓   |  ✓   |  ✓   |  ✓   |      |       |       |      |
| `message.new_chat_members`        |      |      |      |      |      |       |       |      |
| `message.new_chat_title`          |      |      |      |      |      |       |       |      |
| `message.new_chat_photo`          |      |      |      |      |      |       |       |      |
| `message.pinned_message`          |      |      |      |      |      |       |       |      |
| `message.is_service_message`      |      |      |      |      |      |       |       |      |
| `message.is_command`              |      |      |      |      |      |       |       |      |

#### 字段说明

字段是如何设计的？它大致有以下几种类别：

1. 与 Telegram 官方消息结构一致的字段。这样的字段占了大多数，它们的含义也和真实数据中的对应字段相同。
1. 以 `is_` 起头的字段。例如 `message.is_command`。除官方数据中也存在的之外，还特别新增了一些。它们一般可独立构成条件。
1. 扩展的伪字段。这种字段表达的结构可能是错误的但逻辑能成立，例如 `message.text.size`。实际上在真实消息数据中 `text` 是一个字符串，不存在更具体的字段。这里的 `size` 可理解为对 `text` 内容的求总长操作。

#### 运算符说明

下列是对运算符的逐一解释：

- `eq`: 相等（equal）。可匹配数字和字符串的单值。
- `gt`: 大于（greater than）。可匹配数字。
- `ge`: 大于或等于（greater or equal）。可匹配数字。
- `le`: 小于或等于（less or equal）。可匹配数字。
- `in`: 属于其中之一。可匹配字符串/数字的值列表。
- `any`: 包含任意一个。可匹配字符串的值列表。
- `all`: 包含全部，与 `any` 相反。可匹配字符串的值列表。
- `hd`: 头部（head）相等。与 `eq` 类似，但只比较内容的前缀部分而不比较整体。可匹配字符串单值。

#### 一些答疑

- 没有勾选任何运算符的字段怎么使用？答：它表示布尔或非空判断，直接由字段构成条件即可。
- 有大于（`gt`）运算符为什么没有小于？答：因为不需要，前置 `not` 取反即可。

_待补充……_

## 实际案例

本章节将会展示一些常见需求下的规则例子，作为参考方便用户学习。

_待更新……_

## 值表达式

本章节将会介绍一个重要的后续计划，它可以让文本内容匹配更加精准和强大。

_待更新……_

## 性能优化

本章节将会介绍作为开发者，如何使用本库提供的优化相关函数。通过预编译和规则优化，让匹配速度达到极限。

_待更新……_

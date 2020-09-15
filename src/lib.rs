//! Similar to Cloudflare's firewall rules, but used for matching Telegram messages.
//!
//! # Cloudflare 防火墙规则分析
//! 规则可视作多个“条件组”的集合。一般的条件由“字段” + “运算符” + “值” 构成，条件可具备 `and` 或 `or` 关系，不能嵌套。
//!
//! * 在一般条件的构成基础上，前置 `not` 可表示取反。
//! * 字段由多个单词组合而成，通过点（`.`）连接。运算符则使用 snake_case 的风格命名。
//! * 字符串（单）值使用双引号（`""`）包裹，数字值无需引号。
//! * 多值用大括号（`{}`）包裹多个单值，并以空格间隔。多值即「值的列表」。
//! * 相邻的具有 `and` 关系的条件要归纳到同一个括号中，但相邻的 `or` 关系的条件之间彼此独立。
//!
//! 一个具体的例子：
//! ```text
//! (ip.src in {1.1.1.1 192.168.1.1} and not http.request.method in {"GET" "POST"}) or (ip.geoip.country eq "AF") or (http.request.version eq "HTTP/1.1")
//! ```
//! 本库所设计的规则表达式的风格与上述描述保持一致。
//!
//! # 特殊情况：
//! 1. 不具有运算符和值的条件直接使用字段构成，前置 `not` 亦可取反。例如：`(cf.client.bot)`。
//!
//! # **注意**
//! - 当前不支持**特殊情况一**，原因是尚未决定是否采取相同设计。

#![feature(min_specialization)]

pub mod error;
pub mod falsey;
pub mod lexer;
pub mod matcher;
pub mod models;
pub mod ope;
pub mod parser;
pub mod result;
pub mod truthy;

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use matcher::Matcher;
use models::Message;
use result::Result;

/// 使用规则表达式匹配消息。
///
/// # 例子
/// ```
/// use matchingram::rule_match;
/// use matchingram::models::Message;
///
/// let rule = r#"(message.text any {"Hello" "Bye"} and message.text all {"telegram"})"#;
/// let message1 = Message {
///     text: Some(format!("Hello telegram!")),
///     ..Default::default()
/// };
/// let message2 = Message {
///     text: Some(format!("Bye telegram!")),
///     ..Default::default()
/// };
///
/// assert!(rule_match(rule, &message1)?);
/// assert!(rule_match(rule, &message2)?);
/// # Ok::<(), matchingram::Error>(())
/// ```
pub fn rule_match<S: Into<String>>(rule: S, message: &Message) -> Result<bool> {
    let mut matcher = compile_rule(rule)?;

    matcher_match(&mut matcher, message)
}

/// 使用匹配器对象匹配消息。
///
/// 通过 [`compile_rule`](fn.compile_rule.html) 函数编译规则得到匹配器。
pub fn matcher_match(matcher: &mut Matcher, message: &Message) -> Result<bool> {
    matcher.match_message(message)
}

/// 使用匹配器对象匹配消息的 JSON 数据。
#[cfg(feature = "json")]
pub fn matcher_match_json<S: Into<String>>(matcher: &mut Matcher, json_data: S) -> Result<bool> {
    let message: Message = serde_json::from_str(&json_data.into())?;

    matcher.match_message(&message)
}

/// 使用规则表达式匹配消息的 JSON 数据。
///
/// # 例子
/// ```
/// use matchingram::rule_match_json;
///
/// let rule = r#"(message.text any {"Hello" "Bye"} and message.text all {"telegram"})"#;
/// let message1 = r#"{"text": "Hello telegram!"}"#;
/// let message2 = r#"{"text": "Bye telegram!"}"#;
///
/// assert!(rule_match_json(rule, message1)?);
/// assert!(rule_match_json(rule, message2)?);
/// # Ok::<(), matchingram::Error>(())
/// ```
#[cfg(feature = "json")]
pub fn rule_match_json<S1: Into<String>, S2: Into<String>>(rule: S1, json: S2) -> Result<bool> {
    let mut matcher = compile_rule(rule)?;

    matcher_match_json(&mut matcher, json)
}

/// 将字符串表达式规则编译为匹配器对象。
///
/// 详情请参照 [`Matcher::from_rule`](struct.Matcher.html#method.from_rule) 函数。
pub fn compile_rule<S: Into<String>>(rule: S) -> Result<Matcher> {
    Matcher::from_rule(rule)
}

//! Similar to Cloudflare's firewall rules,
//! this library uses specific rules to match Telegram messages.
//! Including keywords, message types, language codes,
//! etc. The conditions in the rules have an `and` or `or` relationship.
//!
//! Cloudflare 防火墙规则分析：每一个条件由“字段” + “运算符” + “值” 构成。条件和条件之间可具备 `and` 或 `or` 的关系，不能嵌套。
//! 字段通过点（`.`）进行分类。运算符使用 snake_case 风格命名。多值使用大括号（`{}`）包裹以及空格分隔，单值使用引号（`""`）包裹。
//! 相邻的具有 `and` 关系的条件会被归纳到同一个括号中，但相邻的 `or` 关系的条件之间彼此独立。
//! 一个具体的例子：
//! ```text
//! (ip.src in {1.1.1.1 192.168.1.1} and http.request.uri.query contains "page") or (ip.geoip.country eq "AF") or (http.request.method eq "POST")
//! ```
//! 本库所设计的规则表达式的风格与之完全一致。

pub mod error;
pub mod lexer;
pub mod matcher;
pub mod models;
pub mod parser;
pub mod result;

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use matcher::Matcher;
use models::Message;
use result::Result;

/// Use the rule expression to match a message.
pub fn rule_match<S: Into<String>>(_rule: S, _message: &Message) -> Result<bool> {
    panic!("This function has not been implemented yet!")
}

/// Use the matcher to match a message.
pub fn matcher_match(_matcher: &Matcher, _message: &Message) -> Result<bool> {
    panic!("This function has not been implemented yet!")
}

/// Use the matcher to match a json data.
pub fn matcher_match_json<S: Into<String>>(_matcher: &Matcher, _json: S) -> Result<bool> {
    panic!("This function has not been implemented yet!")
}

/// Use the rule expression to match a json data.
pub fn rule_match_json<S1: Into<String>, S2: Into<String>>(_rule: S1, _json: S2) -> Result<bool> {
    panic!("This function has not been implemented yet!")
}

/// Compile a string rule expression into a matcher.
/// For details, please refer to [`Matcher::prase`](struct.Matcher.html#method.prase) function.
pub fn compile_rule<S: Into<String>>(rule: S) -> Result<Matcher> {
    Matcher::prase(rule)
}

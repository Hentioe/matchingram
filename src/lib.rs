//! Similar to Cloudflare's firewall rules,
//! this library uses specific rules to match Telegram messages.
//! Including keywords, message types, language codes,
//! etc. Rules can have an `and` or `or` relationship.

pub mod models;
pub mod rule;

use models::Message;
#[doc(inline)]
pub use rule::Rule;

/// Match the message and the rule expression (that is, the string form of the rule).
pub fn match_expression<S: Into<String>>(_message: &Message, _expression: S) -> bool {
    panic!("This function has not been implemented yet!")
}

/// Match the message and the rule.
pub fn match_rule(_message: &Message, _rule: &Rule) -> bool {
    panic!("This function has not been implemented yet!")
}

/// Compile a string expression into a rule.
/// For details, please refer to [`Rule::new`](struct.Rule.html#method.new) function.
pub fn compile_rule<S: Into<String>>(expression: S) -> Rule {
    Rule::new(expression)
}

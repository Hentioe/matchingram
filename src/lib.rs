//! Similar to Cloudflare's firewall rules,
//! this library uses specific rules to match Telegram messages.
//! Including keywords, message types, language codes,
//! etc. Rules can have an `and` or `or` relationship.

pub mod error;
pub mod models;
pub mod result;
pub mod rule;

#[doc(inline)]
pub use error::Error;
use models::Message;
#[doc(inline)]
pub use rule::Rule;

/// Match the message and the expression (that is, the string form of the rule).
pub fn match_expression<S: Into<String>>(_message: &Message, _expression: S) -> bool {
    panic!("This function has not been implemented yet!")
}

/// Match the message and the rule.
pub fn match_rule(_message: &Message, _rule: &Rule) -> bool {
    panic!("This function has not been implemented yet!")
}

/// Compile a string expression into a rule.
/// For details, please refer to [`Rule::prase`](struct.Rule.html#method.prase) function.
pub fn compile_rule<S: Into<String>>(expression: S) -> Result<Rule, Error> {
    Rule::prase(expression)
}

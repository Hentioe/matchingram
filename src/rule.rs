//! Implementation of the rules.

/// Structured rule content.
#[derive(Debug)]
pub struct Rule {}

impl Rule {
    /// Use a string expression to create a rule object.
    /// The string will be expanded into a specific structure,
    /// and the rule object matching will have a faster speed because it does not need to be expanded again.
    pub fn new<S: Into<String>>(_expression: S) -> Self {
        Rule {}
    }
}

//! 全部可能出现的错误。

use super::lexer::Token;
use super::matcher::{Field, Operator};
use thiserror::Error;

/// 错误类别。
#[derive(Debug, Error)]
pub enum Error {
    /// 应该在这里结束。
    #[error("should end here, column: {column:?}")]
    ShouldEndHere { column: usize },

    /// 应该是开启的小括号。
    #[error("should be `(` from column: {column:?}")]
    ShouldOpenParenthesisHere { column: usize },

    /// 应该是关闭的小括号。
    #[error("should be `)` from column: {column:?}")]
    ShouldCloseParenthesisHere { column: usize },

    /// 缺失 token 位置信息。
    #[error("missing token position, index: {index:?}")]
    MissingPosition { index: usize },

    /// 不支持的操作符。
    #[error("the field `{}` does not support the `{}` operator", field.to_string(), operator.to_string())]
    UnsupportedOperator { field: Field, operator: Operator },

    /// 未知的字段。
    #[error("unknown `{field:?}` field")]
    UnknownField { field: String },

    /// 未知的操作符。
    #[error("unknown `{operator:?}` operator")]
    UnknownOperator { operator: String },

    /// 不合法的值。
    #[error("the value `{value:?}` of the field `{field:?}` is invalid")]
    InvalidValue { value: String, field: String },

    /// 缺失字段。
    #[error("missing field from column {column:?}")]
    MissingField { column: usize },

    /// 解析中缺失操作符。
    #[error("missing operator from column {column:?}")]
    MissingOperator { column: usize },

    /// 字段需要运算符。
    #[error("field `{}` requires operator", field.to_string())]
    FieldRequireOperator { field: Field },

    /// 字段需要值。
    #[error("field `{}` requires value", field.to_string())]
    FieldRequireValue { field: Field },

    /// 缺失值。
    #[error("missing value from column {column:?}")]
    MissingValue { column: usize },

    #[error("missing quote from column {column:?}")]
    MissingQuote { column: usize },

    /// 应该是引号。
    #[error("should be `\"` from column: {column:?}")]
    ShouldQuoteHere { column: usize },

    /// 应该是关闭的大括号。
    #[error("should be `}}` from column: {column:?}")]
    ShouldCloseBraceHere { column: usize },

    /// 应该是值。
    #[error("should be values from column: {column:?}")]
    ShouldValueHere { column: usize },

    /// 应该是打开的大括号或引号。
    #[error("should be `{{` or `\"` from column: {column:?}")]
    ShouldOpenBraceOrQuote { column: usize },

    /// 缺失条件。
    #[error("missing condition from column {column:?}")]
    MissingCondition { column: usize },

    /// 位置推断失败。
    #[error("failed to infer position from token `{token:?}`")]
    InferPositionFailed { token: Token },

    /// 缺失 token 位置信息。
    #[error("token `{token:?}` is missing position information, the {position:?}th")]
    MissingTokenPosition { position: usize, token: Token },

    /// 缺失数据。
    #[error("the {index:?}th token data is missing")]
    MissingTokenData { index: usize },

    /// 数字转换出错。
    #[error("error in conversion of numbers starting in column {column:?}")]
    DecimalParseFailed { column: usize },

    /// 解析失败。
    #[error("failed to parse from column {column:?}")]
    ParseFailed { column: usize },

    #[error("{}", source.to_string())]
    #[cfg(feature = "json")]
    Json {
        #[from]
        source: serde_json::Error,
    },
}

//! 全部可能出现的错误。

use super::lexer::Token;
use thiserror::Error;

/// 错误类别。
#[derive(Debug, Error)]
pub enum Error {
    #[error("the first group of conditions is invalid")]
    InvalidFirstGroup,
    /// 不支持的操作符。
    #[error("the field `{field:?}` does not support the `{operator:?}` operator")]
    UnsupportedOperator { field: String, operator: String },
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
    #[error("missing field from column `{column:?}`")]
    MissingField { column: usize },
    /// 缺失操作符。
    #[error("missing operator from column `{column:?}`")]
    MissingOperator { column: usize },
    /// 缺失条件。
    #[error("missing condition from column `{column:?}`")]
    MissingCondition { column: usize },
    /// 位置推断失败。
    #[error("failed to infer position from token `{token:?}`")]
    InferPositionFailed { token: Token },
    /// 缺失 token 位置信息。
    #[error("token `{token:?}` is missing position information, the {position:?}th")]
    MissingTokenPosition { position: usize, token: Token },
    /// 解析失败。
    #[error("failed to parse from column `{column:?}`")]
    ParseFailed { column: usize },
}

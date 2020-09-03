//! 所有可能出现的错误。

use thiserror::Error;

/// 错误类别。
#[derive(Debug, Error)]
pub enum Error {
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
    #[error("failed to parse from column `{column:?}`")]
    ParseFailed { column: usize },
    #[error("missing field from column `{column:?}`")]
    MissingField { column: usize },
    #[error("missing operator from column `{column:?}`")]
    MissingOperator { column: usize },
}

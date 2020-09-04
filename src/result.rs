//! 已包装 Error 的 Result 别名。

use super::error::Error;

/// 包装 [`Error`](../enum.Error.html) 的 Result。
pub type Result<T> = std::result::Result<T, Error>;

use strum_macros::{EnumString, ToString};

pub mod impls;
pub mod prelude;

/// 运算符。
#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumString, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Operator {
    /// 等于。
    Eq,
    /// 大于。
    Gt,
    /// 小于。
    Lt,
    /// 大于或等于。
    Ge,
    /// 小于或等于。
    Le,
    /// 属于其一。
    In,
    /// 包含任意一个。
    Any,
    /// 包含全部。
    All,
    /// 头部相等。
    Hd,
    // 尾部相等。
    Td,
}

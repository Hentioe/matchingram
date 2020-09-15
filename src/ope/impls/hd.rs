/// 运算符 `hd` 的 trait 和相关实现。
use crate::matcher::{RefSingleValue, Values};
use crate::result::Result;

pub trait HdOperator<T> {
    fn hd_ope(&self, target: T) -> Result<bool>;
}

impl HdOperator<&Values> for String {
    fn hd_ope(&self, target: &Values) -> Result<bool> {
        Ok(self.starts_with(target.ref_a_str()?))
    }
}
impl HdOperator<&Values> for Option<String> {
    fn hd_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.hd_ope(target)
        } else {
            Ok(false)
        }
    }
}

/// 运算符 `td` 的 trait 和相关实现。
use crate::matches::{RefSingleValue, Values};
use crate::result::Result;

pub trait TdOperator<T> {
    fn td_ope(&self, target: T) -> Result<bool>;
}

impl TdOperator<&Values> for String {
    fn td_ope(&self, target: &Values) -> Result<bool> {
        Ok(self.ends_with(target.ref_a_str()?))
    }
}
impl TdOperator<&Values> for Option<String> {
    fn td_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.td_ope(target)
        } else {
            Ok(false)
        }
    }
}

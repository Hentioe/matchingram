/// 运算符 `all` 的 trait 和相关实现。
use crate::matcher::{RefSingleValue, Values};
use crate::result::Result;

pub trait AllOperator<T> {
    fn all_ope(&self, target: T) -> Result<bool>;
}

impl AllOperator<&Values> for String {
    fn all_ope(&self, target: &Values) -> Result<bool> {
        let mut result = true;
        for v in target {
            if !self.contains(v.ref_a_str()?) {
                result = false;
                break;
            }
        }

        Ok(result)
    }
}
impl AllOperator<&Values> for Option<String> {
    fn all_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.all_ope(target)
        } else {
            Ok(false)
        }
    }
}

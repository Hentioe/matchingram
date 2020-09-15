/// 运算符 `any` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Values};
use crate::result::Result;

pub trait AnyOperator<T> {
    fn any_ope(&self, target: T) -> Result<bool>;
}

impl AnyOperator<&Values> for String {
    fn any_ope(&self, target: &Values) -> Result<bool> {
        let mut result = false;
        for v in target {
            if self.contains(v.ref_a_str()?) {
                result = true;
                break;
            }
        }

        Ok(result)
    }
}
impl AnyOperator<&Values> for Option<String> {
    fn any_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.any_ope(target)
        } else {
            Ok(false)
        }
    }
}

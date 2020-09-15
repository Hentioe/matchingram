/// 运算符 `in` 的 trait 和相关实现。
use crate::matches::{RefSingleValue, Values};
use crate::result::Result;

pub trait InOperator<T> {
    fn in_ope(&self, target: T) -> Result<bool>;
}

impl InOperator<&Values> for String {
    fn in_ope(&self, target: &Values) -> Result<bool> {
        let mut r = false;

        for v in target {
            if v.ref_a_str()? == self {
                r = true;
                break;
            }
        }

        Ok(r)
    }
}

impl InOperator<&Values> for Option<&String> {
    fn in_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = *self {
            self_data.in_ope(target)
        } else {
            Ok(false)
        }
    }
}

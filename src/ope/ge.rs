/// 运算符 `ge` 的 trait 和相关实现。
use crate::matches::{RefSingleValue, Values};
use crate::result::Result;

pub trait GeOperator<T> {
    fn ge_ope(&self, target: T) -> Result<bool>;
}
pub trait GeOperatorForContentLen<T> {
    fn ge_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl GeOperator<&Values> for i64 {
    fn ge_ope(&self, target: &Values) -> Result<bool> {
        Ok(self >= target.ref_an_integer()?)
    }
}

impl GeOperator<&Values> for i32 {
    fn ge_ope(&self, target: &Values) -> Result<bool> {
        (*self as i64).ge_ope(target)
    }
}

impl GeOperator<&Values> for Option<i32> {
    fn ge_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.ge_ope(target)
        } else {
            Ok(false)
        }
    }
}

impl GeOperator<&Values> for f64 {
    fn ge_ope(&self, target: &Values) -> Result<bool> {
        (*self as i64).ge_ope(target)
    }
}

impl GeOperatorForContentLen<&Values> for String {
    fn ge_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(&self_len >= target.ref_an_integer()?)
    }
}

impl GeOperatorForContentLen<&Values> for Option<String> {
    fn ge_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.ge_ope_for_content_len(target)
        } else {
            Ok(false)
        }
    }
}

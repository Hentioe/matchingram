/// 运算符 `le` 的 trait 和相关实现。
use crate::matches::{RefSingleValue, Values};
use crate::result::Result;

pub trait LeOperator<T> {
    fn le_ope(&self, target: T) -> Result<bool>;
}
pub trait LeOperatorForContentLen<T> {
    fn le_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl LeOperator<&Values> for i64 {
    fn le_ope(&self, target: &Values) -> Result<bool> {
        Ok(self <= target.ref_a_decimal()?)
    }
}

impl LeOperatorForContentLen<&Values> for &String {
    fn le_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(&self_len <= target.ref_a_decimal()?)
    }
}

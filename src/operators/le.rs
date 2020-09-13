/// 运算符 `le` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait LeOperator<T> {
    fn le_ope(&self, target: T) -> Result<bool>;
}
pub trait LeOperatorForContentLen<T> {
    fn le_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl LeOperator<&Vec<Value>> for i64 {
    fn le_ope(&self, target: &Vec<Value>) -> Result<bool> {
        Ok(self <= target.ref_a_decimal()?)
    }
}

impl LeOperatorForContentLen<&Vec<Value>> for &String {
    fn le_ope_for_content_len(&self, target: &Vec<Value>) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(&self_len <= target.ref_a_decimal()?)
    }
}

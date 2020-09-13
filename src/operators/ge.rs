/// 运算符 `ge` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait GeOperator<T> {
    fn ge_ope(&self, target: T) -> Result<bool>;
}
pub trait GeOperatorForContentLen<T> {
    fn ge_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl GeOperator<&Vec<Value>> for i64 {
    fn ge_ope(&self, target: &Vec<Value>) -> Result<bool> {
        Ok(self >= target.ref_a_decimal()?)
    }
}

impl GeOperatorForContentLen<&Vec<Value>> for &String {
    fn ge_ope_for_content_len(&self, target: &Vec<Value>) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(&self_len >= target.ref_a_decimal()?)
    }
}
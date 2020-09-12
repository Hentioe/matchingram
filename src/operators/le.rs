/// 运算符 `le` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait LeOperator<T> {
    fn le_ope(&self, target: T) -> Result<bool>;
}
pub trait LeOperatorForTargetLen<T> {
    fn le_ope_for_target_len(&self, target: T) -> Result<bool>;
}

impl LeOperator<i64> for Vec<Value> {
    fn le_ope(&self, target: i64) -> Result<bool> {
        Ok(self.ref_a_decimal()? >= &target)
    }
}

impl LeOperatorForTargetLen<&String> for Vec<Value> {
    fn le_ope_for_target_len(&self, target: &String) -> Result<bool> {
        let len = target.chars().collect::<Vec<_>>().len();

        Ok(self.ref_a_decimal()? >= &(len as i64))
    }
}

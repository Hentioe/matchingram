/// 运算符 `eq` 的 trait 和相关实现。
use crate::matcher::{TakeAStr, Value};
use crate::result::Result;

pub trait EqOperator<T> {
    fn eq_ope(&self, target: T) -> Result<bool>;
}

impl EqOperator<&String> for Vec<Value> {
    fn eq_ope(&self, target: &String) -> Result<bool> {
        Ok(self.take_a_str()?.eq(target))
    }
}
impl EqOperator<Option<&String>> for Vec<Value> {
    fn eq_ope(&self, target: Option<&String>) -> Result<bool> {
        if let Some(target_data) = target {
            Ok(self.take_a_str()?.eq(target_data))
        } else {
            Ok(false)
        }
    }
}

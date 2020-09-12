/// 运算符 `eq` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait EqOperator<T> {
    fn eq_ope(&self, target: T) -> Result<bool>;
    fn eq_ope_for_target_len(&self, target: T) -> Result<bool>;
}

impl EqOperator<&String> for Vec<Value> {
    fn eq_ope(&self, target: &String) -> Result<bool> {
        Ok(self.ref_a_str()?.eq(target))
    }

    fn eq_ope_for_target_len(&self, target: &String) -> Result<bool> {
        let len = target.chars().collect::<Vec<_>>().len();

        Ok(self.ref_a_decimal()? == &(len as i64))
    }
}
impl EqOperator<Option<&String>> for Vec<Value> {
    fn eq_ope(&self, target: Option<&String>) -> Result<bool> {
        if let Some(target_data) = target {
            self.eq_ope(target_data)
        } else {
            Ok(false)
        }
    }

    fn eq_ope_for_target_len(&self, target: Option<&String>) -> Result<bool> {
        if let Some(target_data) = target {
            self.eq_ope_for_target_len(target_data)
        } else {
            Ok(false)
        }
    }
}

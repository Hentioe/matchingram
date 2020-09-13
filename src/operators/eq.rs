/// 运算符 `eq` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait EqOperator<T> {
    fn eq_ope(&self, target: T) -> Result<bool>;
    fn eq_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl EqOperator<&Vec<Value>> for &String {
    fn eq_ope(&self, target: &Vec<Value>) -> Result<bool> {
        Ok(*self == target.ref_a_str()?)
    }

    fn eq_ope_for_content_len(&self, target: &Vec<Value>) -> Result<bool> {
        let len = self.chars().collect::<Vec<_>>().len();

        Ok(target.ref_a_decimal()? == &(len as i64))
    }
}
impl EqOperator<&Vec<Value>> for Option<&String> {
    fn eq_ope(&self, target: &Vec<Value>) -> Result<bool> {
        if let Some(self_data) = *self {
            self_data.eq_ope(target)
        } else {
            Ok(false)
        }
    }

    fn eq_ope_for_content_len(&self, target: &Vec<Value>) -> Result<bool> {
        if let Some(self_data) = *self {
            self_data.eq_ope_for_content_len(target)
        } else {
            Ok(false)
        }
    }
}

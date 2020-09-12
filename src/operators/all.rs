/// 运算符 `all` 的 trait 和相关实现。
use crate::matcher::{TakeAStr, Value};
use crate::result::Result;

pub trait AllOperator<T> {
    fn all_ope(&self, target: T) -> Result<bool>;
}

impl AllOperator<&String> for Vec<Value> {
    fn all_ope(&self, target: &String) -> Result<bool> {
        let mut result = true;
        for v in self {
            if !target.contains(v.take_a_str()?) {
                result = false;
                break;
            }
        }

        Ok(result)
    }
}
impl AllOperator<Option<&String>> for Vec<Value> {
    fn all_ope(&self, target: Option<&String>) -> Result<bool> {
        if let Some(target_data) = target {
            self.all_ope(target_data)
        } else {
            Ok(false)
        }
    }
}

/// 运算符 `any` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait AnyOperator<T> {
    fn any_ope(&self, target: T) -> Result<bool>;
}

impl AnyOperator<&String> for Vec<Value> {
    fn any_ope(&self, target: &String) -> Result<bool> {
        let mut result = false;
        for v in self {
            if target.contains(v.ref_a_str()?) {
                result = true;
                break;
            }
        }

        Ok(result)
    }
}
impl AnyOperator<Option<&String>> for Vec<Value> {
    fn any_ope(&self, target: Option<&String>) -> Result<bool> {
        if let Some(target_data) = target {
            self.any_ope(target_data)
        } else {
            Ok(false)
        }
    }
}

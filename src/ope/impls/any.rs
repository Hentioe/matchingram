/// 运算符 `any` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait AnyOperator<T> {
    fn any_ope(&self, target: T) -> Result<bool>;
}

impl AnyOperator<&Vec<Value>> for String {
    fn any_ope(&self, target: &Vec<Value>) -> Result<bool> {
        let mut result = false;
        for v in target {
            if self.contains(v.ref_a_str()?) {
                result = true;
                break;
            }
        }

        Ok(result)
    }
}
impl AnyOperator<&Vec<Value>> for Option<String> {
    fn any_ope(&self, target: &Vec<Value>) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.any_ope(target)
        } else {
            Ok(false)
        }
    }
}

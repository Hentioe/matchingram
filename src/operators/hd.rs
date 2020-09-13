/// 运算符 `hd` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait HdOperator<T> {
    fn hd_ope(&self, target: T) -> Result<bool>;
}

impl HdOperator<&Vec<Value>> for String {
    fn hd_ope(&self, target: &Vec<Value>) -> Result<bool> {
        Ok(self.starts_with(target.ref_a_str()?))
    }
}
impl HdOperator<&Vec<Value>> for Option<String> {
    fn hd_ope(&self, target: &Vec<Value>) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.hd_ope(target)
        } else {
            Ok(false)
        }
    }
}

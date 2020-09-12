/// 运算符 `gt` 的 trait 和相关实现。
use crate::matcher::{RefSinleValue, Value};
use crate::result::Result;

pub trait GtOperator<T> {
    fn gt_ope(&self, target: T) -> Result<bool>;
}
pub trait GtOperatorForTargetLen<T> {
    fn gt_ope_for_target_len(&self, target: T) -> Result<bool>;
}

impl GtOperator<i64> for Vec<Value> {
    fn gt_ope(&self, target: i64) -> Result<bool> {
        Ok(self.ref_a_decimal()? < &target)
    }
}

impl GtOperatorForTargetLen<&String> for Vec<Value> {
    fn gt_ope_for_target_len(&self, target: &String) -> Result<bool> {
        let len = target.chars().collect::<Vec<_>>().len();

        Ok(self.ref_a_decimal()? < &(len as i64))
    }
}

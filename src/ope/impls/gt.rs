/// 运算符 `gt` 的 trait 和相关实现。
use crate::matcher::{RefSingleValue, Values};
use crate::result::Result;

pub trait GtOperator<T> {
    fn gt_ope(&self, target: T) -> Result<bool>;
}
pub trait GtOperatorForContentLen<T> {
    fn gt_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl GtOperator<&Values> for i64 {
    fn gt_ope(&self, target: &Values) -> Result<bool> {
        Ok(self > target.ref_a_decimal()?)
    }
}

impl GtOperatorForContentLen<&Values> for &String {
    fn gt_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(&self_len > target.ref_a_decimal()?)
    }
}

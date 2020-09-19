/// 运算符 `gt` 的 trait 和相关实现。
use crate::matches::{RefSingleValue, Values};
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

impl GtOperator<&Values> for i32 {
    fn gt_ope(&self, target: &Values) -> Result<bool> {
        (*self as i64).gt_ope(target)
    }
}

impl GtOperator<&Values> for Option<i32> {
    fn gt_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.gt_ope(target)
        } else {
            Ok(false)
        }
    }
}

impl GtOperator<&Values> for f64 {
    fn gt_ope(&self, target: &Values) -> Result<bool> {
        (*self as i64).gt_ope(target)
    }
}

impl GtOperatorForContentLen<&Values> for String {
    fn gt_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(&self_len > target.ref_a_decimal()?)
    }
}

impl GtOperatorForContentLen<&Values> for Option<String> {
    fn gt_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.gt_ope_for_content_len(target)
        } else {
            Ok(false)
        }
    }
}

/// 运算符 `eq` 的 trait 和相关实现。
use crate::matches::{RefSingleValue, Values};
use crate::result::Result;

pub trait EqOperator<T> {
    fn eq_ope(&self, target: T) -> Result<bool>;
}

pub trait EqOperatorForContentLen<T> {
    fn eq_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl EqOperator<&Values> for i64 {
    fn eq_ope(&self, target: &Values) -> Result<bool> {
        Ok(*self == *target.ref_a_decimal()?)
    }
}

impl EqOperator<&Values> for i32 {
    fn eq_ope(&self, target: &Values) -> Result<bool> {
        (*self as i64).eq_ope(target)
    }
}

impl EqOperator<&Values> for Option<i32> {
    fn eq_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.eq_ope(target)
        } else {
            Ok(false)
        }
    }
}

impl EqOperator<&Values> for String {
    fn eq_ope(&self, target: &Values) -> Result<bool> {
        Ok(*self == target.ref_a_str()?)
    }
}

impl EqOperator<&Values> for Option<String> {
    fn eq_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.eq_ope(target)
        } else {
            Ok(false)
        }
    }
}

impl EqOperatorForContentLen<&Values> for String {
    fn eq_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        let len = self.chars().collect::<Vec<_>>().len();

        Ok(target.ref_a_decimal()? == &(len as i64))
    }
}

impl EqOperatorForContentLen<&Values> for Option<String> {
    fn eq_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.eq_ope_for_content_len(target)
        } else {
            Ok(false)
        }
    }
}

/// 提供判断是否非空或值是否为 `true` 的 trait。
pub trait IsTruthy {
    fn is_truthy(&self) -> bool;
}

// UNSTABLE: min_specialization
impl<T> IsTruthy for Option<T> {
    default fn is_truthy(&self) -> bool {
        if let Some(_v) = self {
            true
        } else {
            false
        }
    }
}

impl IsTruthy for Option<bool> {
    fn is_truthy(&self) -> bool {
        if let Some(v) = *self {
            v
        } else {
            false
        }
    }
}

impl IsTruthy for bool {
    fn is_truthy(&self) -> bool {
        *self
    }
}

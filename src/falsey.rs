use super::error::Error;
use super::result::Result;

pub trait UnwrapOrFalseyHosting<T> {
    fn unwrap_or_falsey_hosting(&self) -> Result<&T>;
}

impl<T> UnwrapOrFalseyHosting<T> for Option<T> {
    fn unwrap_or_falsey_hosting(&self) -> Result<&T> {
        if let Some(v) = self {
            Ok(v)
        } else {
            Err(Error::FalsyValueHosting)
        }
    }
}

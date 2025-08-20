use crate::prompt_mod::error::Error;
use std::str::FromStr;

pub fn parse_as<T>(s: &str) -> Result<T, Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(s).map_err(|e| Error::Parse {
        ty: std::any::type_name::<T>(),
        cause: e.to_string(),
    })
}

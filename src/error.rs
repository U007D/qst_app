use crate::consts::msg;
use std::ffi::OsString;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, 0)]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
}

impl From<std::ffi::OsString> for Error {
    fn from(err: OsString) -> Self {
        Self::ArgNotConvertibleToUtf8(err)
    }
}

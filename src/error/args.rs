use std::ffi::OsString;
use crate::consts::*;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {0:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8)]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[error("{}: {} {0} {} {1}.", msg::ERR_BAD_ARG_COUNT, msg::RECEIVED, msg::BUT_EXPECTED)]
    BadArgCount(usize, usize),
    #[error("{}: {0:?}", msg::ERR_ARG_PARSE)]
    ArgumentParsingError(std::ffi::OsString),
}

impl From<std::ffi::OsString> for Error {
    fn from(oss: std::ffi::OsString) -> Self {
        Self::ArgNotConvertibleToUtf8(oss)
    }
}
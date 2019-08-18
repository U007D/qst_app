use crate::consts::msg;
use snafu::Snafu;

#[derive(Debug, Display)]
pub enum Error {
    #[snafu(display(fmt = "{}: {:?}", "msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8", "source"))]
    ArgNotConvertibleToUtf8 {
        source: std::ffi::OsString},
}

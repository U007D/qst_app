#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod app_args {
    #[derive(Clone, Debug, PartialEq)]
    pub struct AppArgs {
        help: bool,
        number: u32,
        opt_number: Option<u32>,
        width: u32,
        free: Vec<String>,
    }
}
mod consts {
    pub mod msg {
        pub const ERR_ARG_NOT_VALID_USIZE: &str =
            "Error: supplied argument could not be converted to a `usize`";
        pub const ERR_ARG_NOT_POSITIVE: &str =
            "Error: supplied argument must be positive (ie. > 0)";
        pub const ERR_ARGS_PROCESSING: &str = "Error processing args";
    }
}

mod error {
    use crate::consts::msg;
    use derive_more::{Display, From};
    //    use std::str::FromStr;
    use pico_args::Error as PicoArgsError;
    use std::num::ParseIntError;

    #[derive(Debug, Display, From)]
    pub enum Error {
        #[display(fmt = "{}: {:?}", "msg::ERR_ARG_NOT_VALID_USIZE", "_0")]
        ArgInvalidIntegralValue(ParseIntError),
        #[display(fmt = "{}: {}", "msg::ERR_ARG_NOT_POSITIVE", "_0")]
        ArgNonPositiveValue(String),
        #[display(fmt = "{}: {}", "msg::ERR_ARGS_PROCESSING", "_0")]
        ArgsProcessing(PicoArgsError),
    }
}

use pico_args::Arguments;
use std::result::Result as StdResult;
use {app_args::AppArgs, error::Error};

pub type Result<T> = StdResult<T, Error>;

fn parse_width(s: &str) -> Result<u32> {
    s.parse()
        .map_err(|e| Error::ArgInvalidIntegralValue(e))
        .and_then(|w| match w > 0 {
            true => Ok(w),
            false => Err(Error::ArgNonPositiveValue(s.to_string())),
        })
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    println!(
        "{:?}",
        AppArgs {
            help: args.contains(["-h", "--help"]),
            number: args.value_from_str("--number")?.unwrap_or(5),
            opt_number: args.value_from_str("--opt-number")?,
            width: args.value_from_fn("--width", parse_width)?.unwrap_or(10),
            free: args.free()?,
        }
    );

    Ok(())
}

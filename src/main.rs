#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// To use `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(bare_trait_objects, unsafe_code)]
#![forbid(overflowing_literals)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license file, and more
//#![warn(clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]
#![allow(clippy::match_bool)]

mod args;
mod consts;
mod error;
mod foo;

use std::result::Result as StdResult;
use structopt::StructOpt;
pub use {args::Args, consts::*, error::Error};
pub use foo::foo;

pub type Result<T> = StdResult<T, Error>;

fn main() -> Result<()> {
    let _args = Args::from_args();
    Ok(())
}

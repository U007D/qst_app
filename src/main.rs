use std::env;

use he_std::args::ParseArgs;

use lib::{self, args::Args, Error, Result};

#[termination::display]
fn main() -> Result<()> {
    let _args = Args::try_parse(env::args_os()).map_err(Error::from)?;

    Ok(())
}

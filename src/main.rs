#[macro_use]
extern crate failure;
#[macro_use]
extern crate xfailure;

mod config;
mod errors;
mod symbols;

use crate::config::*;
use crate::errors::*;

fn main() -> Result<(), BRError> {
    let config = Config::parse_cmdline()?;
    let symbols = symbols::exported_symbols(config.input_path)?;
    symbols.dump(config.output_dir)?;
    Ok(())
}

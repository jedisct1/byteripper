extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate xfailure;
extern crate goblin;
extern crate libloading;

mod config;
mod errors;
mod symbols;

use config::*;
use errors::*;

fn main() -> Result<(), BRError> {
    let config = Config::parse_cmdline()?;
    let symbols = symbols::exported_symbols(config.input_path)?;
    symbols.dump(config.output_dir)?;
    Ok(())
}

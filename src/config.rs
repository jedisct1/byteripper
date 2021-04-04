use crate::errors::*;
use clap::{App, Arg};
use std::path::PathBuf;

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn parse_cmdline() -> Result<Self, BRError> {
        let matches = App::new("byteripper")
            .version("1.0")
            .about("A tool to extract code from individual functions in a library")
            .arg(
                Arg::with_name("input_file")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .required(true)
                    .help("Path to the input file"),
            )
            .arg(
                Arg::with_name("output_dir")
                    .short("o")
                    .long("output-dir")
                    .takes_value(true)
                    .required(true)
                    .help("Path to the output directory"),
            )
            .get_matches();
        let input_path = PathBuf::from(
            matches
                .value_of("input_file")
                .ok_or(BRError::UsageError("Input file required"))?,
        );
        let output_dir = PathBuf::from(
            matches
                .value_of("output_dir")
                .ok_or(BRError::UsageError("Output directory required"))?,
        );
        let config = Config {
            input_path,
            output_dir,
        };
        Ok(config)
    }
}

use std::path::PathBuf;

use clap::{Arg, Command};

use crate::errors::*;

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn parse_cmdline() -> Result<Self, BRError> {
        let matches = Command::new("byteripper")
            .version("1.0")
            .about("A tool to extract code from individual functions in a library")
            .arg(
                Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Path to the input file"),
            )
            .arg(
                Arg::new("output_dir")
                    .short('o')
                    .long("output-dir")
                    .required(true)
                    .help("Path to the output directory"),
            )
            .get_matches();
        let input_path = PathBuf::from(
            matches
                .get_one::<String>("input_file")
                .ok_or(BRError::UsageError("Input file required"))?,
        );
        let output_dir = PathBuf::from(
            matches
                .get_one::<String>("output_dir")
                .ok_or(BRError::UsageError("Output directory required"))?,
        );
        let config = Config {
            input_path,
            output_dir,
        };
        Ok(config)
    }
}

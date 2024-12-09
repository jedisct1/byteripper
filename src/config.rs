use std::path::PathBuf;

use clap::Arg;

use crate::errors::*;

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn parse_cmdline() -> Result<Self, BRError> {
        let matches = clap::command!()
            .arg(
                Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .takes_value(true)
                    .required(true)
                    .help("Path to the input file"),
            )
            .arg(
                Arg::new("output_dir")
                    .short('o')
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

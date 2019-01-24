use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::config::parser::parse_config;
use crate::config::validator::validate_config;
use crate::config::FileConfig;

pub fn load_config(file_path: &str) -> FileConfig {
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to open file {}.\n{}", file_path, err);
        std::process::exit(1);
    });

    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .expect("Failed to read from file");

    let config = match parse_config(&content) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    };

    match validate_config(&config) {
        Ok(warnings) => {
            for warning in warnings {
                eprintln!("WARNING: {}", warning);
            }
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    }

    config
}

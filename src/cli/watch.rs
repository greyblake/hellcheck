use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc;

use crate::config::FileConfig;
use crate::config_parser::parse_config;
use crate::config_validator::validate_config;

use crate::reactor::StateMessage;

#[derive(StructOpt, Debug)]
pub struct WatchOpts {
    #[structopt(short = "f", long = "file")]
    file: String,
}

pub fn run(opts: WatchOpts) {
    let config = load_config(&opts.file);

    let (sender, receiver) = mpsc::channel::<StateMessage>();
    crate::reactor::spawn(receiver, config.clone());
    crate::watcher::run(config, sender);
}

fn load_config(file_path: &str) -> FileConfig {
    let file = File::open(file_path).unwrap();
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

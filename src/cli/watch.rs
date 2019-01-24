use structopt::StructOpt;

use std::sync::mpsc;

use crate::cli::helpers::load_config;
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

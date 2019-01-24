use structopt::clap::AppSettings;
use structopt::StructOpt;

mod test;
mod watch;

use test::TestOpts;
use watch::WatchOpts;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
enum Command {
    /// Watch monitoring
    #[structopt(name = "watch")]
    Watch(WatchOpts),

    /// Test checkers and notifiers
    #[structopt(name = "test")]
    Test(TestOpts),
}

pub fn run() {
    let command = Command::from_args();

    match command {
        Command::Watch(opts) => watch::run(opts),
        Command::Test(opts) => test::run(opts),
    };
}

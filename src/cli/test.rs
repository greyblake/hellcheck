use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct TestOpts {
    #[structopt(short = "f", long = "file")]
    file: String,
}

pub fn run(_opts: TestOpts) {
    eprintln!("test command is not implemented yet");
}

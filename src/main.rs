use minigrep::Opt;
use std::process;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = minigrep::run(opt) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}

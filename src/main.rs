use clap::Parser;
use fuzz::config::Cli;
use std::process;

fn main() {
    let config = Cli::parse();
    config.validate().unwrap_or_else(|err| {
        println!("parsing arguments problem: {err}");
        process::exit(1);
    });
    if let Err(err) = fuzz::run(config) {
        println!("application error: {err}");
        process::exit(1);
    }
    process::exit(0)
}

use std::{env, process};
use fuzz::config::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Args = Args::build(&args).unwrap_or_else(|err| {
        println!("parsing arguments problem: {err}");
        process::exit(1);
    });
    if let Err(err) = fuzz::run(config) {
        println!("application error: {err}");
        process::exit(1);
    }
    process::exit(1)
}

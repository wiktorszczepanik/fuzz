use fuzz::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = fuzz::Config::build(&args).unwrap_or_else(|err| {
        println!("parsing arguments problem: {err}");
        process::exit(1);
    });
    if let Err(err) = fuzz::run(config) {
        println!("application error: {err}");
        process::exit(1);
    }
    process::exit(1)
}

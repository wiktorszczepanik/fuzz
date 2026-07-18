use std::error::Error;
use std::fs;
use crate::config::Args;

pub mod config;
mod tests;

pub fn run(config: Args) -> Result<(), Box<dyn Error>> {
    let _content = fs::read_to_string(config.file_path)
        .expect("cannot read file content");
    Ok(())
}

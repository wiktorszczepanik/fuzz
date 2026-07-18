use crate::config::Cli;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use crate::search::Search;

pub mod config;
pub mod search;

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let _content = collect_file_content(&config.file_path);
    let _search = Search::new();
    Ok(())
}

fn collect_file_content(file: &PathBuf) -> Result<String, &'static str> {
    let content = fs::read_to_string(file)
        .expect("cannot read file content");
    Ok(content)
}

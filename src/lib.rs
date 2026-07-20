use crate::config::Cli;
use crate::search::Search;
use std::error::Error;

pub mod config;
pub mod display;
pub mod search;

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let mut search = Search::new()?;
    search.index_lines(config.file_path)?;
    let results = search.query(config.text, config.top)?;
    display::show(results, config.lines, config.score);
    Ok(())
}

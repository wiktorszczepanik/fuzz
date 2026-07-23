use crate::config::Cli;
use crate::score::Score;
use crate::search::Search;
use std::error::Error;
use crate::filter::Filter;

pub mod config;
pub mod display;
pub mod score;
pub mod search;
pub mod filter;

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let mut search = Search::new()?;
    search.index_lines(config.file_path)?;
    let mut results = search.query(&config.text)?;
    Score::calculate(config.text, &mut results);
    Filter::apply(config.top, &mut results);
    display::show(results, config.lines, config.score);
    Ok(())
}

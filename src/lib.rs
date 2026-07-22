use crate::config::Cli;
use crate::score::Score;
use crate::search::Search;
use std::error::Error;

pub mod config;
pub mod display;
pub mod score;
pub mod search;

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let mut search = Search::new()?;
    search.index_lines(config.file_path)?;
    let mut results = search.query(&config.text, config.top)?;
        Score::calculate(config.text, &mut results);
        sort_results(&mut results);
        display::show(results, config.lines, config.score);
    Ok(())
}

fn sort_results(results: &mut Vec<(usize, f64, String)>) {
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
}

use crate::config::Cli;
use std::error::Error;
use tantivy::Score;
use crate::search::Search;

pub mod config;
pub mod search;

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let mut search = Search::new()?;
    search.add_lines(config.file_path)?;
    let results = search.query(config.text, config.top);
    Ok(())
}

fn display(results: Vec<(Score, String)>, lines_numbers: bool, score: bool) {
    for (score, line) in results {
        println!("{}", line)
    }
}

// fn collect_file_content(file: &PathBuf) -> Result<String, &'static str> {
//     let content = fs::read_to_string(file)
//         .expect("cannot read file content");
//     Ok(content)
// }

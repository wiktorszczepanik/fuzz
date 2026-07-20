use tantivy::Score;

pub fn show(results: Vec<(Score, String)>, lines_numbers: bool, score: bool) {
    match (lines_numbers, score) {
        (false, false) => display_plain(results),
        (true, false) => display_with_line_numbers(results),
        (false, true) => display_with_score(results),
        (true, true) => display_with_line_numbers_and_score(results),
    }
}

fn display_plain(results: Vec<(Score, String)>) {
    for (_, line) in results {
        println!("{}", line)
    }
}

fn display_with_line_numbers(results: Vec<(Score, String)>) {
    for (line_number, (_, line)) in results.into_iter().enumerate() {
        println!("{}: {}", line_number + 1, line);
    }
}

fn display_with_score(results: Vec<(Score, String)>) {
    for (score, line) in results {
        println!("[{score:.3}] {line}");
    }
}
fn display_with_line_numbers_and_score(results: Vec<(Score, String)>) {
    let mut line_number: usize = 1;
    for (score, line) in results {
        println!("{line_number}: [{score:.3}] {line}");
        line_number += 1;
    }
}

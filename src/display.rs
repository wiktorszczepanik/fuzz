pub fn show(results: Vec<(usize, f64, String)>, lines_numbers: bool, score: bool) {
    match (lines_numbers, score) {
        (false, false) => println!("{}", display_plain(results)),
        (true, false) => println!("{}", display_with_line_numbers(results)),
        (false, true) => println!("{}", display_with_score(results)),
        (true, true) => println!("{}", display_with_line_numbers_and_score(results)),
    }
}

fn display_plain(results: Vec<(usize, f64, String)>) -> String {
    results
        .into_iter()
        .map(|(_, _, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn display_with_line_numbers(results: Vec<(usize, f64, String)>) -> String {
    results
        .into_iter()
        .map(|(line_number, _, line)| format!("{}: {}", line_number, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn display_with_score(results: Vec<(usize, f64, String)>) -> String {
    results
        .into_iter()
        .map(|(_, score, line)| format!("[{score:.3}] {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn display_with_line_numbers_and_score(results: Vec<(usize, f64, String)>) -> String {
    results
        .into_iter()
        .map(|(line_number, score, line)| format!("{}: [{score:.3}] {}", line_number, line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::display::{
        display_plain, display_with_line_numbers, display_with_line_numbers_and_score,
        display_with_score,
    };

    fn results() -> Vec<(usize, f64, String)> {
        vec![
            (10, 1.23456, "Lorem ipsum".to_string()),
            (25, 2.0, "Dolor sit".to_string()),
        ]
    }

    #[test]
    fn display_plain_should_show_only_lines() {
        assert_eq!(display_plain(results()), "Lorem ipsum\nDolor sit");
    }

    #[test]
    fn display_with_line_numbers_should_show_real_line_numbers() {
        assert_eq!(
            display_with_line_numbers(results()),
            "10: Lorem ipsum\n25: Dolor sit"
        );
    }

    #[test]
    fn display_with_score_should_show_scores() {
        assert_eq!(
            display_with_score(results()),
            "[1.235] Lorem ipsum\n[2.000] Dolor sit"
        );
    }

    #[test]
    fn display_with_line_numbers_and_score_should_show_both() {
        assert_eq!(
            display_with_line_numbers_and_score(results()),
            "10: [1.235] Lorem ipsum\n25: [2.000] Dolor sit"
        );
    }
}

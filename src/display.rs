const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
// const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

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
        .map(|(line_number, _, line)| format!("{GREEN}{line_number}:{RESET} {line}"))
            .collect::<Vec<_>>()
            .join("\n")
}

fn display_with_score(results: Vec<(usize, f64, String)>) -> String {
    results
        .into_iter()
        .map(|(_, score, line)| format!("{YELLOW}[{score:07.3}]{RESET} {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn display_with_line_numbers_and_score(results: Vec<(usize, f64, String)>) -> String {
    results
        .into_iter()
        .map(|(line_number, score, line)| format!("{GREEN}{line_number}:{RESET} {YELLOW}[{score:07.3}]{RESET} {line}"))
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
            "\u{1b}[32m10:\u{1b}[0m Lorem ipsum\n\u{1b}[32m25:\u{1b}[0m Dolor sit"
        );
    }

    #[test]
    fn display_with_score_should_show_scores() {
        assert_eq!(
            display_with_score(results()),
            "\u{1b}[33m[001.235]\u{1b}[0m Lorem ipsum\n\u{1b}[33m[002.000]\u{1b}[0m Dolor sit"
        );
    }

    #[test]
    fn display_with_line_numbers_and_score_should_show_both() {
        assert_eq!(
            display_with_line_numbers_and_score(results()),
            "\u{1b}[32m10:\u{1b}[0m \u{1b}[33m[001.235]\u{1b}[0m Lorem ipsum\n\u{1b}[32m25:\u{1b}[0m \u{1b}[33m[002.000]\u{1b}[0m Dolor sit"
        );
    }
}

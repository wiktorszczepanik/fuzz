pub struct Filter {}

impl Filter {
    pub fn apply(top: u8, results: &mut Vec<(usize, f64, String)>) {
        Self::sort_results(results);
        let limit = Self::calculate_top_lines(top, results.len());
        results.truncate(limit);
    }

    fn sort_results(results: &mut Vec<(usize, f64, String)>) {
        results.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    fn calculate_top_lines(top_percent: u8, lines: usize) -> usize {
        let top = top_percent as usize;
        std::cmp::max(1, (top * lines + 99) / 100)
    }
}

#[cfg(test)]
mod tests {
    use crate::filter::Filter;

    #[test]
    fn calculate_top_lines_should_round_up() {
        assert_eq!(Filter::calculate_top_lines(10, 100), 10);
        assert_eq!(Filter::calculate_top_lines(1, 100), 1);
        assert_eq!(Filter::calculate_top_lines(50, 3), 2);
        assert_eq!(Filter::calculate_top_lines(25, 7), 2);
        assert_eq!(Filter::calculate_top_lines(100, 5), 5);
    }

    #[test]
    fn calculate_top_lines_should_never_return_zero() {
        assert_eq!(Filter::calculate_top_lines(0, 100), 1);
        assert_eq!(Filter::calculate_top_lines(0, 0), 1);
        assert_eq!(Filter::calculate_top_lines(50, 0), 1);
    }

    #[test]
    fn sort_results_should_sort_by_score_descending() {
        let mut results = vec![
            (1, 10.5, "low".to_string()),
            (2, 50.0, "high".to_string()),
            (3, 25.0, "medium".to_string()),
        ];
        Filter::sort_results(&mut results);
        assert_eq!(results[0].1, 50.0);
        assert_eq!(results[1].1, 25.0);
        assert_eq!(results[2].1, 10.5);
    }

    #[test]
    fn apply_should_return_only_top_percent_results() {
        let mut results = vec![
            (1, 10.0, "low".to_string()),
            (2, 50.0, "high".to_string()),
            (3, 30.0, "medium".to_string()),
            (4, 20.0, "lower".to_string()),
        ];
        Filter::apply(50, &mut results);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, 50.0);
        assert_eq!(results[1].1, 30.0);
    }

    #[test]
    fn apply_should_sort_before_truncating() {
        let mut results = vec![
            (1, 10.0, "low".to_string()),
            (2, 90.0, "highest".to_string()),
            (3, 50.0, "medium".to_string()),
        ];
        Filter::apply(33, &mut results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, 90.0);
    }

    #[test]
    fn apply_should_keep_at_least_one_result() {
        let mut results = vec![
            (1, 10.0, "only".to_string()),
        ];
        Filter::apply(0, &mut results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, 10.0);
    }

    #[test]
    fn apply_should_handle_empty_results() {
        let mut results: Vec<(usize, f64, String)> = vec![];
        Filter::apply(50, &mut results);
        assert!(results.is_empty());
    }
}

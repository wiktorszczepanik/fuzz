pub struct Score {}

impl Score {
    pub fn calculate(
        query: String,
        lines: &mut Vec<(usize, f64, String)>,
    ) -> &Vec<(usize, f64, String)> {
        for (_, score, line) in lines.iter_mut() {
            *score = Self::calculate_line(&query, line);
        }
        lines
    }

    fn calculate_line(query: &str, line: &str) -> f64 {
        let mut score = 0.0;
        if query.is_empty() || line.is_empty() {
            return score;
        }
        // Exact phrase
        Self::calculate_exact_start_values(&mut score, query, line);
        // Lowercase phrase
        let lower_query = query.to_lowercase();
        let lower_line = line.to_lowercase();
        Self::calculate_exact_lowercase_values(&mut score, &lower_query, &lower_line);
        // Token matching
        let query_tokens: Vec<&str> = query.split_whitespace().collect();
        Self::calculate_any_exact_token_match(&mut score, &query_tokens, line);
        let lower_query_tokens: Vec<&str> = lower_query.split_whitespace().collect();
        Self::calculate_any_lowercase_token_match(&mut score, &lower_query_tokens, &lower_line);
        // Token order
        Self::calculate_order_bonus(&mut score, &lower_query_tokens, &lower_line);
        // Token distance
        Self::calculate_distance_bonus(&mut score, &lower_query_tokens, &lower_line);
        // Word boundaries
        Self::calculate_word_boundary_bonus(&mut score, &lower_query_tokens, &lower_line);
        // Character subsequence
        Self::calculate_subsequence_bonus(&mut score, &lower_query, &lower_line);
        // Prefer shorter lines
        Self::calculate_length_penalty(&mut score, &lower_query, &lower_line);
        score
    }

    fn calculate_exact_start_values(score: &mut f64, query: &str, line: &str) {
        if query == line {
            *score += 100.0;
        }
        if line.starts_with(query) {
            *score += 95.0;
        }
        if line.contains(query) {
            *score += 90.0;
        }
    }

    fn calculate_exact_lowercase_values(score: &mut f64, query: &str, line: &str) {
        if query == line {
            *score += 80.0;
        }
        if line.starts_with(query) {
            *score += 75.0;
        }
        if line.contains(query) {
            *score += 70.0;
        }
    }

    fn calculate_any_exact_token_match(score: &mut f64, tokens: &[&str], line: &str) {
        for token in tokens {
            if line.contains(token) {
                *score += 25.0;
            }
        }
    }

    fn calculate_any_lowercase_token_match(score: &mut f64, tokens: &[&str], line: &str) {
        for token in tokens {
            if line.contains(token) {
                *score += 15.0;
            }
        }
    }

    fn calculate_order_bonus(score: &mut f64, tokens: &[&str], line: &str) {
        let mut last_pos = 0;
        let mut bonus = 5.0;
        for token in tokens {
            if let Some(pos) = line[last_pos..].find(token) {
                *score += bonus;
                bonus += 5.0;
                last_pos += pos + token.len();
            } else {
                break;
            }
        }
    }

    fn calculate_distance_bonus(score: &mut f64, tokens: &[&str], line: &str) {
        let mut positions = Vec::new();
        for token in tokens {
            if let Some(pos) = line.find(token) {
                positions.push(pos);
            }
        }
        if positions.len() < 2 {
            return;
        }
        let first = *positions.first().unwrap();
        let last = *positions.last().unwrap();
        let distance = last.saturating_sub(first);
        *score += 50.0 / (distance as f64 + 1.0);
    }

    fn calculate_word_boundary_bonus(score: &mut f64, tokens: &[&str], line: &str) {
        for token in tokens {
            if let Some(pos) = line.find(token) {
                if pos == 0 {
                    *score += 10.0;
                    continue;
                }
                if let Some(prev) = line[..pos].chars().last() {
                    if !prev.is_alphanumeric() {
                        *score += 10.0;
                    }
                }
            }
        }
    }

    fn calculate_subsequence_bonus(score: &mut f64, query: &str, line: &str) {
        let mut query_chars = query.chars();
        let mut current = match query_chars.next() {
            Some(c) => c,
            None => return,
        };
        let mut consecutive = 0;
        for char in line.chars() {
            if char == current {
                consecutive += 1;
                *score += 3.0 + consecutive as f64;
                if let Some(next) = query_chars.next() {
                    current = next;
                } else {
                    break;
                }
            } else {
                consecutive = 0;
            }
        }
    }

    fn calculate_length_penalty(score: &mut f64, query: &str, line: &str) {
        let diff = line.len().saturating_sub(query.len());
        if diff > 0 {
            *score -= diff as f64 * 0.10;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::score::Score;

    fn calculate(query: &str, line: &str) -> f64 {
        Score::calculate_line(query, line)
    }

    #[test]
    fn empty_query_returns_zero() {
        assert_eq!(calculate("", "hello world"), 0.0);
    }

    #[test]
    fn empty_line_returns_zero() {
        assert_eq!(calculate("hello", ""), 0.0);
    }

    #[test]
    fn exact_match_scores_higher_than_partial_match() {
        let exact = calculate("hello", "hello");
        let partial = calculate("hello", "hello world");
        assert!(exact > partial);
    }

    #[test]
    fn exact_case_sensitive_match_gets_bonus() {
        let exact = calculate("hello", "hello");
        let different_case = calculate("hello", "Hello");
        assert!(exact > different_case);
    }

    #[test]
    fn lowercase_match_is_case_insensitive() {
        let lower = calculate("hello", "HELLO");
        assert!(lower > 0.0);
    }

    #[test]
    fn token_matching_increases_score() {
        let one_token = calculate("hello", "hello world");
        let two_tokens = calculate("hello world", "hello world");
        assert!(two_tokens > one_token);
    }

    #[test]
    fn token_order_gives_bonus() {
        let correct_order = calculate("hello world", "hello world");
        let wrong_order = calculate("hello world", "world hello");
        assert!(correct_order > wrong_order);
    }

    #[test]
    fn closer_tokens_score_higher() {
        let close = calculate("hello world", "hello world");
        let far = calculate("hello world", "hello very very very long world");
        assert!(close > far);
    }

    #[test]
    fn word_boundary_scores_better_than_inside_word_match() {
        let boundary = calculate("cat", "cat dog");
        let inside_word = calculate("cat", "concatenate");
        assert!(boundary > inside_word);
    }

    #[test]
    fn subsequence_matching_works() {
        let subsequence = calculate("abc", "a_b_c");
        assert!(subsequence > 0.0);
    }

    #[test]
    fn shorter_lines_are_preferred() {
        let short = calculate("rust", "rust");
        let long = calculate("rust", "rust programming language book");
        assert!(short > long);
    }

    #[test]
    fn unrelated_text_scores_lower() {
        let match_score = calculate("rust", "rust programming");
        let unrelated_score = calculate("rust", "javascript frontend");
        assert!(match_score > unrelated_score);
    }

    #[test]
    fn exact_match_has_high_score() {
        let score = calculate("lorem ipsum", "lorem ipsum");
        assert!(score > 200.0, "expected strong exact match, got {}", score);
    }
}

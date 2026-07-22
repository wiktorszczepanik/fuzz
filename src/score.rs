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

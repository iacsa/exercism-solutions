use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut counts = HashMap::new();
    for c in candidate
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
    {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts.values().all(|&v| v <= 1)
}

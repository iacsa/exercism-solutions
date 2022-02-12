pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len <= digits.len() {
        (0..=digits.len() - len)
            .map(|i| digits[i..i + len].to_string())
            .collect()
    } else {
        vec![]
    }
}

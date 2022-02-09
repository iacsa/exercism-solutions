pub fn is_pangram(string: &str) -> bool {
    let string = string.to_lowercase();
    ('a'..='z').all(|c| string.contains(c))
}

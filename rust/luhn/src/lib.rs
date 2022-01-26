/// Check a Luhn checksum.
pub fn is_valid(string: &str) -> bool {
    parse(string)
        .filter(|digits| digits.len() > 1)
        .map(checksum)
        .filter(|sum| sum % 10 == 0)
        .is_some()
}

fn parse(string: &str) -> Option<Vec<u32>> {
    string
        .replace(" ", "")
        .chars()
        .map(|c| c.to_digit(10))
        .collect()
}

fn checksum(digits: Vec<u32>) -> i64 {
    digits.into_iter().rev().enumerate().map(value).sum()
}

fn value((index, digit): (usize, u32)) -> i64 {
    (digit as i64 * (1 + (index as i64 & 1)) - 1) % 9 + 1
}

use std::cmp::max;

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        count += 1;
        if chars.peek() != Some(&c) {
            if count > 1 {
                result += &format!("{}", count);
            }
            result.push(c);
            count = 0;
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    for c in source.chars() {
        if let Some(d) = c.to_digit(10) {
            count = count * 10 + d;
            continue;
        }
        for _ in 0..max(count, 1) {
            result.push(c);
        }
        count = 0;
    }
    result
}

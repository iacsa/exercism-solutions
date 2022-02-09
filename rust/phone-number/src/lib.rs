pub fn number(s: &str) -> Option<String> {
    let digits: Vec<_> = s.chars().filter(|c| c.is_digit(10)).collect();
    let digits = if digits[0] == '1' {
        &digits[1..]
    } else {
        &digits[..]
    };

    (digits.len() == 10
        && digits[0] != '0'
        && digits[0] != '1'
        && digits[3] != '0'
        && digits[3] != '1')
        .then(|| digits.iter().collect())
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|num| num[..3].to_string())
}

pub fn pretty_print(s: &str) -> String {
    number(s)
        .map(|num| format!("({}) {}-{}", &num[..3], &num[3..6], &num[6..]))
        .unwrap_or("invalid".to_string())
}

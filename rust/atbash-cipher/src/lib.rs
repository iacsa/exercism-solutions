fn reverse(c: char) -> char {
    if c.is_ascii_alphabetic() {
        (b'z' + b'a' - (c as u8)) as char
    } else {
        c
    }
}

pub fn encode(text: &str) -> String {
    let alphanum: Vec<char> = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(reverse)
        .collect();
    // waiting for Iterator::intersperse to stabilize
    alphanum
        .chunks(5)
        .collect::<Vec<_>>()
        .join(&' ')
        .iter()
        .collect()
}

pub fn decode(cipher: &str) -> String {
    cipher.chars().filter(|&c| c != ' ').map(reverse).collect()
}

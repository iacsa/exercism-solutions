pub fn abbreviate(name: &str) -> String {
    let mut result = String::new();
    let mut last = ' ';
    for current in name.chars() {
        if current.is_alphabetic()
            && (!last.is_alphabetic() && last != '\''
                || current.is_uppercase() && last.is_lowercase())
        {
            result.push(current);
        }
        last = current;
    }
    result.to_uppercase()
}

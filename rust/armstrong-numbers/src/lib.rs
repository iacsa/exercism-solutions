pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let number_of_digits = digits.len() as u32;
    num == digits.iter().map(|n| n.pow(number_of_digits)).sum()
}

fn digits(mut num: u32) -> Vec<u32> {
    let mut result = vec![];
    while num > 0 {
        result.push(num % 10);
        num /= 10;
    }
    result
}

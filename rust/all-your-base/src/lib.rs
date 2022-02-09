use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDigit(u32),
    InvalidInputBase,
    InvalidOutputBase,
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if let Some(&d) = number.iter().find(|&&d| d >= from_base) {
        return Err(Error::InvalidDigit(d));
    }

    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut value = number.iter().fold(0, |acc, digit| acc * from_base + digit);

    let mut result = VecDeque::new();
    loop {
        result.push_front(value % to_base);
        value = value / to_base;
        if value == 0 {
            break;
        }
    }

    Ok(Vec::from(result))
}

use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Error {
    Overflow,
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&value| {
            let mut result = VecDeque::with_capacity(5);
            let mut v = value;
            result.push_front((v % 128) as u8);
            while v >= 128 {
                v /= 128;
                result.push_front((v % 128) as u8 | 128);
            }
            result
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if !bytes.is_empty() && *bytes.last().unwrap() & 128 != 0 {
        return Err(Error::IncompleteNumber);
    }
    let mut result = Vec::new();
    let mut value: u32 = 0;
    for &byte in bytes {
        let checked_value = value.checked_mul(128).ok_or(Error::Overflow)?;
        // after the multiplication, adding a value < 127 cannot overflow
        value = checked_value + byte as u32 % 128;
        // first bit being 0 represents the last segment for this value
        if byte & 128 == 0 {
            result.push(value);
            value = 0;
        }
    }

    Ok(result)
}

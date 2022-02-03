#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u32, Error> {
    let digits = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<_>, _>>()?;

    // Vec::windows panics if span is zero
    if span == 0 {
        return Ok(1);
    }

    digits
        .windows(span)
        .map(|series| series.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}

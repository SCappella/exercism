#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if let Some(c) = string_digits.chars().find(|c| !c.is_numeric()) {
        return Err(Error::InvalidDigit(c));
    }

    let digits: Vec<u64> = string_digits
        .chars()
        .map(|c| c.to_digit(10).expect("Test for numerics failed").into())
        .collect();
    if let Some(max) = digits
        .windows(span)
        .map(|window| window.iter().product())
        .max()
    {
        Ok(max)
    } else {
        Err(Error::SpanTooLong)
    }
}

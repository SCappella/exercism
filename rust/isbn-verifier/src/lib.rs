/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Vec<_> = isbn
        .chars()
        .scan(0, |count, c| {
            if *count > 10 {
                return None;
            }

            if c == 'X' && *count == 9 {
                *count += 1;
                return Some(Some(10));
            }

            if let Some(d) = c.to_digit(10) {
                *count += 1;
                Some(Some(d))
            } else {
                Some(None)
            }
        })
        .filter_map(|d| d)
        .collect();

    if digits.len() != 10 {
        return false;
    }

    digits
        .into_iter()
        .zip(1..=10)
        .fold(0, |acc, (dig, mul)| (acc + dig * mul) % 11)
        == 0
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits = Vec::new();
    for c in code.chars() {
        if c == ' ' {
            continue;
        }

        if let Some(digit) = c.to_digit(10) {
            digits.push(digit)
        } else {
            return false;
        }
    }

    if digits.len() <= 1 {
        return false;
    }

    digits
        .into_iter()
        .rev()
        .zip(std::iter::repeat(&[1, 2]).flatten())
        .map(|(x, y)| x * y)
        .map(|x| if x < 10 { x } else { x - 9 })
        .sum::<u32>()
        % 10
        == 0
}

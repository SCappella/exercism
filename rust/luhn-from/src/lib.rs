pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut digits = Vec::new();
        for c in self.code.chars() {
            match c {
                ' ' => continue,
                '0'...'9' => digits.push(c.to_digit(10).unwrap()),
                _ => return false,
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
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}

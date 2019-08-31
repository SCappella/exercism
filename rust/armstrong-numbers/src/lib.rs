const BASE: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = (0..)
        .scan(num, |num, _| {
            if *num == 0 {
                return None;
            }
            let digit = *num % BASE;
            *num /= BASE;
            Some(digit)
        })
        .collect();

    let len = digits.len() as u32;
    digits.into_iter().map(|d| d.pow(len)).sum::<u32>() == num
}

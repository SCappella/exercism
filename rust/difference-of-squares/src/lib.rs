pub fn square_of_sum(n: u32) -> u32 {
    n.pow(2) * (n + 1).pow(2) / 4
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    if n > 0 {
        n * (n - 1) * (n + 1) * (3 * n + 2) / 12
    } else {
        0
    }
}

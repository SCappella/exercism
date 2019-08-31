const SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    if !(1 <= s && s <= SQUARES) {
        panic!("Square must be between 1 and {}", SQUARES)
    }
    2u64.pow(s - 1)
}

// u64 uschmixty-four
pub fn total() -> u128 {
    // This will be simpler
    2u128.pow(SQUARES) - 1
}

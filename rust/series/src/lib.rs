pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        // kinda arbitrary, but whatever
        return vec!["".to_owned(); digits.len() + 1];
    }

    let chars: Vec<_> = digits.chars().collect();

    chars
        .windows(len)
        .map(|series| series.iter().collect())
        .collect()
}

pub fn check(candidate: &str) -> bool {
    let letters: Vec<_> = candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let letters_dedup: std::collections::BTreeSet<_> = letters.iter().collect();

    letters.len() == letters_dedup.len()
}

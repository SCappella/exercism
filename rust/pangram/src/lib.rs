use std::collections::BTreeSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let letters: BTreeSet<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    sentence
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>()
        == letters
}

use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in words
        .split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .filter(|word| !word.is_empty())
    {
        let word = word.trim_matches('\'').to_lowercase().to_owned();
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

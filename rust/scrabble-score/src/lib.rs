use std::collections::BTreeMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score_chart = BTreeMap::new();
    for &c in &['a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'] {
        score_chart.insert(c, 1);
    }
    for &c in &['d', 'g'] {
        score_chart.insert(c, 2);
    }
    for &c in &['b', 'c', 'm', 'p'] {
        score_chart.insert(c, 3);
    }
    for &c in &['f', 'h', 'v', 'w', 'y'] {
        score_chart.insert(c, 4);
    }
    for &c in &['k'] {
        score_chart.insert(c, 5);
    }
    for &c in &['j', 'x'] {
        score_chart.insert(c, 8);
    }
    for &c in &['q', 'z'] {
        score_chart.insert(c, 10);
    }

    word.chars()
        .map(|c| c.to_ascii_lowercase())
        .filter_map(|c| score_chart.get(&c))
        .sum()
}
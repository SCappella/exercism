use std::collections::BTreeMap;

pub fn transform(scores: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_scores = BTreeMap::new();
    for (&score, chars) in scores.iter() {
        for &c in chars {
            new_scores.insert(c.to_ascii_lowercase(), score);
        }
    }

    new_scores
}

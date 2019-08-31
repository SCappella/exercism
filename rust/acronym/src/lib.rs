pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_ascii_alphanumeric())
        // split each word at interior capital letters (e.g. HyperText -> Hyper Text)
        .flat_map(|word| {
            if word.chars().all(|c| c.is_ascii_uppercase()) {
                vec![word.to_owned()]
            } else {
                word.match_indices(|c: char| c.is_ascii_uppercase())
                    .map(|(index, _)| index)
                    .chain(std::iter::once(word.len()))
                    .scan(
                        (0, word.chars()),
                        |(prev_index, remaining_chars), new_index| {
                            let new_chars = new_index - *prev_index;
                            *prev_index = new_index;
                            Some(remaining_chars.by_ref().take(new_chars).collect::<String>())
                        },
                    )
                    .collect()
            }
        })
        .filter_map(|word| word.chars().next().map(|c| c.to_ascii_uppercase()))
        .collect()
}

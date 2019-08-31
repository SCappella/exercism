pub fn encode(source: &str) -> String {
    source
        .chars()
        // bwahahaha
        // this allows the scan to recognize the end of the string
        .chain(std::iter::once('\0'))
        .scan((None, 0), |(curr_char, count), next_char| {
            if *curr_char == Some(next_char) {
                *count += 1;
                Some(None)
            } else if let Some(curr_char_unwrap) = *curr_char {
                let output = (curr_char_unwrap, *count);
                *curr_char = Some(next_char);
                *count = 1;
                Some(Some(output))
            } else {
                *curr_char = Some(next_char);
                *count = 1;
                Some(None)
            }
        })
        .filter_map(|x| {
            x.map(|(c, count)| {
                if count == 1 {
                    format!("{}", c)
                } else {
                    format!("{}{}", count, c)
                }
            })
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .scan(0, |count, c| match c.to_digit(10) {
            Some(d) => {
                *count *= 10;
                *count += d as usize;
                Some(None)
            }
            None => {
                if *count == 0 {
                    // this means we had an implicit "1c"
                    Some(Some(c.to_string()))
                } else {
                    let output = c.to_string().repeat(*count);
                    *count = 0;
                    Some(Some(output))
                }
            }
        })
        .filter_map(|x| x)
        .collect()
}

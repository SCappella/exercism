fn is_shouted(string: &str) -> bool {
    string.chars().all(|c| !c.is_ascii_lowercase()) &&
    string.chars().any(|c| c.is_ascii_uppercase())
}

fn is_blank(string: &str) -> bool {
    string.chars().all(|c| c.is_whitespace())
}

fn is_question(string: &str) -> bool {
    string.chars().rev().skip_while(|c| c.is_whitespace()).next() == Some('?')
}

pub fn reply(message: &str) -> &str {
    if is_blank(message) {
        "Fine. Be that way!"
    } else if is_question(message) {
        if is_shouted(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if is_shouted(message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

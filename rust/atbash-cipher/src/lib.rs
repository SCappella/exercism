fn encode_char(c: char) -> Option<char> {
    if c.len_utf8() == 1 {
        match c as u8 {
            // note the lower case a-------------------v
            c @ b'A'...b'Z' => Some((b'Z' - c + b'a').into()),
            c @ b'a'...b'z' => Some((b'z' - c + b'a').into()),
            b'0'...b'9' => Some(c),
            _ => None,
        }
    } else {
        None
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(encode_char)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(encode_char).collect()
}

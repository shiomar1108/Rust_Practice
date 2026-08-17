pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                let lower = c.to_ascii_lowercase() as u8;
                Some((b'z' - lower + b'a') as char)
            } else if c.is_ascii_digit() {
                Some(c)
            } else {
                None
            }
        })
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i.is_multiple_of(5) {
                vec![' ', c]
            } else {
                vec![c]
            }
        })
        .collect()
}

pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let lower = c.to_ascii_lowercase() as u8;
                (b'z' - lower + b'a') as char
            } else {
                c
            }
        })
        .collect()
}
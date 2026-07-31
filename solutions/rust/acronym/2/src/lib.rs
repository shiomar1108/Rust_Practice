pub fn abbreviate(phrase: &str) -> String {
    let mut acron = String::new();
    let mut prev_char: Option<char> = None;

    for c in phrase.chars() {
        let is_prev_word_boundary = prev_char.is_none_or(|p| p.is_whitespace() || p == '-' || p == '_');
        let is_prev_lowercase = prev_char.is_some_and(|p| p.is_lowercase());

        if c.is_alphabetic() 
            && is_prev_word_boundary || (c.is_uppercase() && is_prev_lowercase) {
                acron.push(c.to_ascii_uppercase());
            }
        prev_char = Some(c);
    }
    acron
}
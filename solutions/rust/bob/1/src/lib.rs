pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() { return "Fine. Be that way!"; }
    let is_question = trimmed.ends_with('?');
    let contains_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = contains_letters && !trimmed.chars().any(|c| c.is_lowercase());
    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => "Whatever.",
    }
}




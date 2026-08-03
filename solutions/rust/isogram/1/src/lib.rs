pub fn check(candidate: &str) -> bool {
    for l in candidate.chars(){
        if candidate.to_lowercase().chars().filter(|&c| c == l && c.is_alphabetic()).count() > 1 {
            return false;
        }
    }
    true
}

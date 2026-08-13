pub fn number(user_number: &str) -> Option<String> {
    
    let mut digits: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 11 && digits.starts_with('1') {
        digits.remove(0);
    }
    if digits.len() != 10 {
        return None;
    }
    
let bytes = digits.as_bytes();
    let area_code_valid = matches!(bytes[0], b'2'..=b'9');
    let exchange_code_valid = matches!(bytes[3], b'2'..=b'9');

    if area_code_valid && exchange_code_valid {
        Some(digits)
    } else {
        None
    }
}

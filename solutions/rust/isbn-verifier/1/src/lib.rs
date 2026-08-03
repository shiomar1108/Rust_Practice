pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digits = isbn.chars().filter_map(|c| match c {
        '0'..='9' => c.to_digit(10),
        'X' => Some(10),
        '-' => None,
        _ => Some(11),
    });

    let mut sum = 0;
    for position in (1..=10).rev() {
        match digits.next() {
            Some(10) if position != 1 => return false, 
            Some(val) if val <= 10 => sum += val * position,
            _ => return false,
        }
    }
    digits.next().is_none() && sum % 11 == 0
}
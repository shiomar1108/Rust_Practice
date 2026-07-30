/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut digits_count = 0;
    for (i,c) in code.chars().rev().filter(|c| !c.is_whitespace()).enumerate() {
        match c.to_digit(10){
            Some(mut digit) => {
                if !i.is_multiple_of(2) {
                    digit *=2;
                     if digit > 9 {
                        digit -= 9;
                    }
                }
            sum +=digit;
            digits_count += 1;
        }
        None => return false,
    }
    }
    digits_count > 1 && sum.is_multiple_of(10)
}

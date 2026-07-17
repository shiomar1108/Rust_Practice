fn is_prime(num: u32) -> bool {
    if num <= 1 { return false; }
    if num == 2 || num == 3 { return true; }
    if num.is_multiple_of(2) || num.is_multiple_of(3) { return false; }

    // Check divisors up to the square root of the number
    let mut i = 5;
    while i * i <= num {
        if num.is_multiple_of(i) || num.is_multiple_of(i+2) {
            return false;
        }
        i += 6;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|&c| is_prime(c)).nth(n as usize).unwrap()
}

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`)
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    mmi(a)?;
    let v = coder(plaintext, |x| a * ((x as i32 | 32) - 97) + b);
    Ok(String::from_utf8(v.chunks(5).collect::<Vec<_>>().join(&32)).unwrap())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`)
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let n = mmi(a)?;
    Ok(String::from_utf8(coder(ciphertext, |y| n * (y as i32 - 97 - b))).unwrap())
}

fn mmi(a: i32) -> Result<i32, AffineCipherError> {
    (1..26).find(|x| a * x % 26 == 1).ok_or(AffineCipherError::NotCoprime(a))
}

fn coder(s: &str, f: impl Fn(u8) -> i32) -> Vec<u8> {
    s.bytes()
        .filter(u8::is_ascii_alphanumeric)
        .map(|x| match x {
            65.. => f(x).rem_euclid(26) as u8 + 97,
            _ => x,
        }).collect()
}
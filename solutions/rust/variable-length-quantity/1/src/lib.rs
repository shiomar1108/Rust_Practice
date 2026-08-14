#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    for value in values {
        let mut bytes = vec![(value & 0x7f) as u8];
        let mut number: u32 = value >> 7;
        while number != 0 {
            bytes.insert(0, (number & 0x7f | 0x80) as u8);
            number >>= 7;
        }
        result.extend(bytes);
    }
    result
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut number: u32 = 0;
    for (idx, byte) in bytes.iter().enumerate() {
        if number & 0xfe_00_00_00 > 0 {
            return Err(Error::Overflow);
        }
        number <<= 7;
        number |= u32::from(byte & 0x7f);
        if 0x80 & byte == 0 {
            result.push(number);
            number = 0;
        } else if idx + 1 == bytes.len() {
            return Err(Error::IncompleteNumber);
        }
    }
    Ok(result)
}
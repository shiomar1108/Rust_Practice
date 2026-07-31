#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut res = vec![];
    let mut value = 0;
    if from_base <= 1 { return Err(Error::InvalidInputBase);}
    if to_base <= 1 { return Err(Error::InvalidOutputBase); }
    for (i, d) in number.iter().rev().enumerate() {
        if d >= &from_base {return Err(Error::InvalidDigit(*d)); }
        value += d * from_base.pow(i as u32)
    }
    while value > 0 {
        res.push(value % to_base);
        value /= to_base;
    }
    res.reverse();
    if res.is_empty() {
        res.push(0);
    }
    Ok(res)
} 


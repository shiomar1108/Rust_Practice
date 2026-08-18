use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mappings = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut value = String::new();

        // Greedily subtract values from highest to lowest
        for &(val, symbol) in &mappings {
            while num >= val {
                value.push_str(symbol);
                num -= val;
            }
        }

        Self { value }
    }
}
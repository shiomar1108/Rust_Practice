static LETTER_VALUES: [(char, u64); 26] = [
    ('A', 1), ('E', 1), ('I', 1), ('O', 1), ('U', 1),
    ('L', 1), ('N', 1), ('R', 1), ('S', 1), ('T', 1),
    ('D', 2), ('G', 2),
    ('B', 3), ('C', 3), ('M', 3), ('P', 3),
    ('F', 4), ('H', 4), ('V', 4), ('W', 4), ('Y', 4),
    ('K', 5),
    ('J', 8), ('X', 8),
    ('Q', 10), ('Z', 10),
];
pub fn score(word: &str) -> u64 {
    let mut points: u64  = 0;
    for c in  word.to_ascii_uppercase().chars() {
       points += LETTER_VALUES.iter().find(|&&(k, _)| k == c).map(|&(_, v)| v).unwrap_or(0);
    }
    points
}

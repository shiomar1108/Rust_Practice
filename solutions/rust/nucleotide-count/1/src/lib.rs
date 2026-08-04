use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    const VALID: [char; 4] = ['A', 'C', 'G', 'T'];

    if !VALID.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let map = nucleotide_counts(dna)?;
    Ok(*map.get(&nucleotide).unwrap_or(&0))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .into_iter()
        .collect();
    for c in dna.chars() {
        match res.get_mut(&c) {
            Some(count) => *count += 1,
            None => return Err(c),
        }
    }
    Ok(res)
}

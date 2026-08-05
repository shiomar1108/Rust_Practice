const VALID: [char; 4] = ['A', 'C', 'G', 'T'];
const VALID_RNA: [char; 4] = ['A', 'C', 'G', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna { sequence: String }

#[derive(Debug, PartialEq, Eq)]
pub struct Rna { sequence: String }

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(index) = dna.chars().position(|c| !VALID.contains(&c)) {
            Err(index)
        } else {
            Ok(Dna {
                sequence: dna.to_string(),
            })
        }
    }

    pub fn into_rna(self) -> Rna {
        let sequence = self
            .sequence
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => unreachable!("Dna struct is guaranteed to have valid characters"),
            })
            .collect();

        Rna { sequence }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(index) = rna.chars().position(|c| !VALID_RNA.contains(&c)) {
            Err(index)
        } else {
            Ok(Rna {
                sequence: rna.to_string(),
            })
        }
    }
}
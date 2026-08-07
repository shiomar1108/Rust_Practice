const STOP: [&str; 3] = ["UAA", "UAG", "UGA"];

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut res : Option<Vec<&str>> = Some(vec![]);

    if !rna.len().is_multiple_of(3) && !STOP.iter().any(|&word| rna.contains(word))  { return None; }

     for c in rna.as_bytes().chunks(3) {
         println! ("c: {:?}", c);
         let sequence = match c {
             b"AUG" => "Methionine",
             b"UUU" | b"UUC" => "Phenylalanine",
             b"UUA" | b"UUG" => "Leucine",
             b"UCU" | b"UCC" | b"UCA" | b"UCG" => "Serine",
             b"UAU" | b"UAC" => "Tyrosine",
             b"UGU" | b"UGC" => "Cysteine",
             b"UGG" => "Tryptophan",
             b"UAA" | b"UAG" | b"UGA" => break,
              _ => return None,
         };
         println! ("sequence: {}", sequence);
         res.get_or_insert_with(Vec::new).push(sequence);
         }
    res
}

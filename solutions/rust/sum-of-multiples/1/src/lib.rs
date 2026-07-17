pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();
    for &val in factors{
        if val == 0 { continue; }
        multiples.extend((val..limit).step_by(val as usize));
    }
    multiples.sort_unstable();
    multiples.dedup();
    println!("{:?}",multiples);
    multiples.iter().sum()
}

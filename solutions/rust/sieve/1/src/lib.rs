pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    if upper_bound < 2 {
        return res;
    }
    'outer: for n in 2..=upper_bound {
        let limit = (n as f64).sqrt() as u32;
        for i in 2..=limit {
            if n.is_multiple_of(i.into()) {
                continue 'outer; 
            }
        }
        res.push(n);
    }
    res
}

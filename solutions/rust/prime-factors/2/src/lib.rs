pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut candidate = 2;

    // A composite number must have a prime factor less than or equal to its square root
    while candidate * candidate <= n {
        while n.is_multiple_of(candidate) {
            prime_factors.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }

    // If anything remains greater than 1, it must be a prime number
    if n > 1 {
        prime_factors.push(n);
    }

    prime_factors
}

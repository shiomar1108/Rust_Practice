pub fn collatz(n: u64) -> Option<u64> {
    let mut steps: Option<u64> =None;
    let mut temp = n;
    if n == 0 { return None; }
    if n== 1 {return Some(0); }

    while temp != 1{
        if temp.is_multiple_of(2) {
            temp /= 2;
        } else {
            temp *= 3;
            temp += 1;
        }
         *steps.get_or_insert(0) += 1;
    }
    
    steps
}

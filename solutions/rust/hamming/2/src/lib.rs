
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut diff: Option<usize> = None;
    if s1.len() == s2.len() {
        if  s1.is_empty() {
            diff = Some(0)
        } else {
            diff = Some(0);
            for i in 0..s1.len(){
                if s1.chars().nth(i) != s2.chars().nth(i) {
                    *diff.get_or_insert(0) += 1; 
                }
            }
        }
    }
    diff
}

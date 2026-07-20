pub fn build_proverb(list: &[&str]) -> String {
    let mut res: String = String::new();
    let mut iter = list.iter().peekable(); 
    while let Some(current) = iter.next() {
         if let Some(next) = iter.peek() {
            res.push_str(&format!("For want of a {} the {} was lost.\n",current,next));
         } else {
             res.push_str(&format!("And all for the want of a {}.",list[0]));
         }
    }
    res
}

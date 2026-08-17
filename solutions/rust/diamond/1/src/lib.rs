pub fn get_diamond(c: char) -> Vec<String> {
    let target = c.to_ascii_uppercase();
    if !target.is_ascii_uppercase() {
        return Vec::new();
    }

    let target_idx = (target as u8 - b'A') as usize;
    let mut top_half: Vec<String> = (0..=target_idx)
        .map(|i| {
            let ch = (b'A' + i as u8) as char;
            let outer_padding = " ".repeat(target_idx - i);

            if i == 0 {
                format!("{outer_padding}{ch}{outer_padding}")
            } else {
                let inner_padding = " ".repeat(2 * i - 1);
                format!("{outer_padding}{ch}{inner_padding}{ch}{outer_padding}")
            }
        })
        .collect();
    let bottom_half: Vec<String> = top_half.iter().rev().skip(1).cloned().collect();
    top_half.extend(bottom_half);
    top_half
}
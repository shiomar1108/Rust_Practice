pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut my_string = String::new();
    for count in 0..take_down {
        let before = start_bottles - count;
        let after = before - 1;
        my_string.push_str(&format!("{} green {} hanging on the wall,\n", capitalize(number_to_word(before)), bottle_str(before)));
        my_string.push_str(&format!("{} green {} hanging on the wall,\n", capitalize(number_to_word(before)), bottle_str(before)));
        my_string.push_str("And if one green bottle should accidentally fall,\n");
        my_string.push_str(&format!("There'll be {} green {} hanging on the wall.", number_to_word(after), bottle_str(after)));

        if count < take_down {
            my_string.push_str("\n\n");
        }
    }
    my_string
}

fn bottle_str(n: u32) -> &'static str {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
}

fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    } 
    let (first, rest) = s.split_at(1);
    format!("{}{}", first.to_uppercase(), rest)
}

fn number_to_word(num: u32) -> &'static str {
    match num {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "",
    }
}
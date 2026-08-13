pub fn encrypt(input: &str) -> String {
    let text = input.chars().filter(|x| x.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
    let c = (text.len() as f64).sqrt().ceil() as usize;
    let r = if c == 0 { 0 } else { (text.len() as f64 / c as f64).ceil() as usize };
    println!("{},{}",r,c);
    (0..c).map(|x| {
            (0..r).map(|y| {
                    text.as_bytes()
                        .get(x + y * c)
                        .map(|&b| b as char)
                        .unwrap_or(' ')
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
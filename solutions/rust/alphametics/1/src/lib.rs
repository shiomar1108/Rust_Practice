use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use std::iter::once;

fn string_to_value(string: &str, letters_value: &HashMap<char, u8>) -> u32 {
    string.chars().rev().enumerate().map(|(i, v)| *letters_value.get(&v).unwrap() as u32 * 10_u32.pow(i as u32)).sum()
}


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (adders_str, result_str) = match input.split(" == ").collect::<Vec<&str>>() {
        v if v.len() == 2 => (v[0].split(" + ").collect::<Vec<&str>>(), v[1]),
        _ => panic!()
    };
    let unique_letters: Vec<char> = result_str.chars().chain(adders_str.iter().flat_map(|s| s.chars())).collect::<HashSet<char>>().into_iter().collect();
    let leading_letters: Vec<char> = adders_str.iter().chain(once(&result_str)).filter_map(|s| s.chars().next()).collect();
    let N_unique_letters = unique_letters.len();

    for vals in (0..10).map(|e| e as u8).permutations(N_unique_letters) {
        let letters_value: HashMap<char, u8> = unique_letters.iter().zip(vals.into_iter()).map(|(&l, v)| (l, v)).collect();
        if leading_letters.iter().any(|c| *letters_value.get(c).unwrap() ==0) {continue}  // no leading zero
        
        let adders_sum: u32 = adders_str.iter().map(|s| string_to_value(s, &letters_value)).sum();
        let result_sum: u32 = string_to_value(result_str, &letters_value);
        if adders_sum == result_sum {
            return Some(letters_value)
        }
    }
    None
}
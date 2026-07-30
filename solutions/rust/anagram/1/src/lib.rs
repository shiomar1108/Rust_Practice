use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut words_resulted: HashSet<&'a str> = HashSet::new();
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort();
    for anagram in possible_anagrams {
        if anagram.to_lowercase() == word.to_lowercase() {
            continue
        }
        let mut compared_world: Vec<char> = anagram
            .to_lowercase()
            .chars()
            .collect();
        compared_world.sort();
        if word_chars == compared_world {
            words_resulted.insert(anagram);
        }
    }
    words_resulted
}
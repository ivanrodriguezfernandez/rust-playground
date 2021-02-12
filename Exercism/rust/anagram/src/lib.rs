use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word_lower = to_lower_chars(word);
    let mut word_sorted = word_lower.to_vec();
    word_sorted.sort_unstable();

    for item in possible_anagrams.iter() {
        let item_lower = to_lower_chars(item);
        if item_lower.len() != word_lower.len() || item_lower == word_lower {
            continue;
        }
        let mut item_sorted = item_lower.to_vec();
        item_sorted.sort_unstable();
        if item_sorted == word_sorted {
            result.insert(*item);
        }
    }
    result
}

fn to_lower_chars(word: &str) -> Vec<char> {
    word.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().nth(0).unwrap()
            } else {
                c
            }
        })
        .collect::<Vec<char>>()
}
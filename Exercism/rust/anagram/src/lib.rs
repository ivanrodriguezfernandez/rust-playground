use std::{collections::HashSet, iter::FromIterator};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    for item in possible_anagrams.iter() {
        
        if item.to_lowercase() != word.to_lowercase()
            && ordering(&item.to_lowercase()) == ordering(&word.to_lowercase())
        {
            let s: &'a str = &item;
            result.insert(s);
        }
    }
    result
}

pub fn ordering(w: &str) -> String {
    let mut chars: Vec<char> = w.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(&chars)
}

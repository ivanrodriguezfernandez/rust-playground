use std::{collections::HashSet, iter::FromIterator};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    
    let word_prepared = ordering(&word.to_lowercase());
    let mut result = HashSet::new();

    for i in 0..possible_anagrams.len(){
        if  ordering(&possible_anagrams[i].to_lowercase()) == word_prepared {
            let m = possible_anagrams[i].clone();
            result.insert(m);
        }
    }
    return result;

}

pub fn ordering(w:&str) -> String {
    let mut chars: Vec<char> = w.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let s = String::from_iter(&chars);
    return s;
}
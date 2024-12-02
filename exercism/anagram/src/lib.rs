use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_map = word_to_map(word);
    let mut anagrams: HashSet<&str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        if *possible_anagram.to_lowercase() != word.to_lowercase()
            && is_anagram(char_map.clone(), possible_anagram)
        {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}

fn word_to_map(word: &str) -> HashMap<char, usize> {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    // building a hashmap for character: count
    for c in word.chars() {
        let c = c.to_lowercase().next().unwrap();
        let count = char_map.entry(c).or_insert(0);
        *count += 1;
    }

    char_map
}

fn is_anagram(mut char_map: HashMap<char, usize>, possible_anagrams: &str) -> bool {
    for c in possible_anagrams.chars() {
        let c = c.to_lowercase().next().unwrap();
        if *char_map.get(&c).unwrap_or(&0) == 0 {
            return false;
        }
        char_map.entry(c).and_modify(|e| *e -= 1);
    }

    // if all values are 0 at the end
    char_map.values().all(|e| *e == 0)
}

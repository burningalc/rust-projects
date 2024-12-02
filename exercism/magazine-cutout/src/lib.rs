use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_map = HashMap::new();
    for &word in magazine.iter() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    for &word in note.iter() {
        let count = word_map.entry(word).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }

    true
}

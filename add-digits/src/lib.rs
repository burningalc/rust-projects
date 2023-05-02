use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut current_num = num;
        let mut cache: HashMap<i32, i32> = HashMap::new();
        while current_num / 10 != 0 {
            current_num = match cache.get(&current_num) {
                Some(n) => *n,
                None => {
                    let new_cache = current_num
                        .to_string()
                        .chars()
                        .map(|d| d.to_digit(10).unwrap() as i32)
                        .collect::<Vec<i32>>()
                        .iter()
                        .sum::<i32>();
                    cache.entry(current_num).or_insert(new_cache);
                    new_cache
                }
            };
        }
        current_num
    }
}

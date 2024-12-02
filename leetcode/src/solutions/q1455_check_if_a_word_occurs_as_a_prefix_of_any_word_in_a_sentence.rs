/* LeetCode Problem Question 1455: Check If a Word Occurs As a Prefix of Any Word in a Sentence

Given a sentence that consists of some words separated by a single space, and a searchWord, check if searchWord is a prefix of any word in sentence.

Return the index of the word in sentence (1-indexed) where searchWord is a prefix of this word. If searchWord is a prefix of more than one word, return the index of the first word (minimum index). If there is no such word return -1.

A prefix of a string s is any leading contiguous substring of s.

https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
*/

pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        match sentence
            .split(" ")
            .enumerate()
            .find(|(_, word)| word.starts_with(&search_word))
        {
            Some((idx, _)) => idx as i32 + 1,
            None => -1,
        }
    }
}

#[test]
fn test_1() {
    let input_sentence = "i love eating burger".to_string();
    let input_search_word = "burg".to_string();
    let expected_output = 4;

    assert_eq!(
        Solution::is_prefix_of_word(input_sentence, input_search_word),
        expected_output
    )
}

#[test]
fn test_2() {
    let input_sentence = "this problem is an easy problem".to_string();
    let input_search_word = "pro".to_string();
    let expected_output = 2;

    assert_eq!(
        Solution::is_prefix_of_word(input_sentence, input_search_word),
        expected_output
    )
}

#[test]
fn test_3() {
    let input_sentence = "i am tired".to_string();
    let input_search_word = "you".to_string();
    let expected_output = -1;

    assert_eq!(
        Solution::is_prefix_of_word(input_sentence, input_search_word),
        expected_output
    )
}

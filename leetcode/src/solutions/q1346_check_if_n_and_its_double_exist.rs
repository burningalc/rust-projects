/* LeetCode Problem Question 1346: Check If N and Its Double Exist

Given an array arr of integers, check if there exist two indices i and j such that :
    i != j
    0 <= i, j < arr.length
    arr[i] == 2 * arr[j]

https://leetcode.com/problems/check-if-n-and-its-double-exist
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();

        seen.insert(arr[0]);

        for &element in arr.iter().skip(1) {
            if seen.contains(&(element * 2)) {
                return true;
            }

            if element % 2 == 0 && seen.contains(&(element / 2)) {
                return true;
            }

            seen.insert(element);
        }

        false
    }
}

#[test]
fn test_1() {
    let input = vec![10, 2, 5, 3];
    let expected_output = true;

    assert_eq!(expected_output, Solution::check_if_exist(input));
}

#[test]
fn test_2() {
    let input = vec![3, 1, 7, 11];
    let expected_output = false;

    assert_eq!(expected_output, Solution::check_if_exist(input));
}

#[test]
fn test_3() {
    let input = vec![7, 1, 14, 11];
    let expected_output = true;

    assert_eq!(expected_output, Solution::check_if_exist(input));
}

/* LeetCode Problem Question 0839: Similar String Groups

Two strings, X and Y, are considered similar if either they are identical or we can make them equivalent by swapping at most two letters (in distinct positions) within the string X.

For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar, but "star" is not similar to "tars", "rats", or "arts".

Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.  Notice that "tars" and "arts" are in the same group even though they are not similar.  Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.

We are given a list strs of strings where every string in strs is an anagram of every other string in strs. How many groups are there?

https://leetcode.com/problems/similar-string-groups/
*/

use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut groups: Vec<usize> = (0..strs.len()).collect();

        // iterate the strs and assign each to their group
        for i in 0..strs.len() {
            // compare all strs before index i
            for j in 0..i {
                if Self::is_similar(&strs[i], &strs[j]) {
                    Self::union_root(&mut groups, i, j)
                }
            }
        }

        // report how many groups there are,
        // by use a hash set to filter duplicates
        let unique_groups = (0..strs.len())
            .map(|i| Self::find_root(&groups, i))
            .collect::<HashSet<usize>>();
        unique_groups.len() as i32
    }

    pub fn is_similar(a: &String, b: &String) -> bool {
        if a == b {
            return true;
        }

        let mut misplaced_count = 0;
        for (char_a, char_b) in a.chars().zip(b.chars()) {
            if char_a != char_b {
                misplaced_count += 1;
            }
            if misplaced_count == 3 {
                return false;
            }
        }
        true
    }

    pub fn union_root(groups: &mut [usize], i: usize, j: usize) {
        // connect i and j in the disjoint set by union-find algorithm
        let root_i = Self::find_root(groups, i);
        let root_j = Self::find_root(groups, j);
        groups[root_i] = root_j;
    }

    pub fn find_root(groups: &[usize], node: usize) -> usize {
        let mut current_node = node;
        loop {
            if groups[current_node] == current_node {
                return current_node;
            } else {
                current_node = groups[current_node];
            }
        }
    }
}

#[test]
fn test_1() {
    let input = ["tars", "rats", "arts", "star"];
    let input = input.iter().map(|f| f.to_string()).collect();
    let expected_output = 2;

    assert_eq!(Solution::num_similar_groups(input), expected_output);
}

#[test]
fn test_2() {
    let input = ["omv", "ovm"];
    let input = input.iter().map(|f| f.to_string()).collect();
    let expected_output = 1;

    assert_eq!(Solution::num_similar_groups(input), expected_output);
}

#[test]
fn test_3() {
    let input = [
        "kccomwcgcs",
        "socgcmcwkc",
        "sgckwcmcoc",
        "coswcmcgkc",
        "cowkccmsgc",
        "cosgmccwkc",
        "sgmkwcccoc",
        "coswmccgkc",
        "kowcccmsgc",
        "kgcomwcccs",
    ];
    let input = input.iter().map(|f| f.to_string()).collect();
    let expected_output = 5;

    assert_eq!(Solution::num_similar_groups(input), expected_output);
}

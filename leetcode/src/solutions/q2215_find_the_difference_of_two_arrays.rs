/* LeetCode Problem Question 2215: Find the Difference of Two Arrays

Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:

answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
Note that the integers in the lists may be returned in any order.

https://leetcode.com/problems/find-the-difference-of-two-arrays/
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1 = nums1.into_iter().collect::<HashSet<i32>>();
        let nums2 = nums2.into_iter().collect::<HashSet<i32>>();

        vec![
            nums1.difference(&nums2).copied().collect(),
            nums2.difference(&nums1).copied().collect(),
        ]
    }
}

#[test]
fn test_1() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    let expected_output = vec![HashSet::from([1, 3]), HashSet::from([4, 6])];

    assert_eq!(
        Solution::find_difference(nums1, nums2)
            .iter()
            .map(|f| f.iter().cloned().collect::<HashSet<i32>>())
            .collect::<Vec<HashSet<i32>>>(),
        expected_output
    );
}

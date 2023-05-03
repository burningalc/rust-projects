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

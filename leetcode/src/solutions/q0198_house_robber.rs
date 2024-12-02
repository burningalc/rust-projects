/* LeetCode Problem Question 0198: House Robber

You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

https://leetcode.com/problems/house-robber/
*/

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        };
        let mut cache = vec![0; nums.len()];
        cache[0] = nums[0];
        cache[1] = std::cmp::max(nums[0], nums[1]);

        for (mut index, num) in nums.iter().skip(2).enumerate() {
            index += 2;
            cache[index] = std::cmp::max(num + cache[index - 2], cache[index - 1]);
        }

        *cache.last().unwrap()
    }
}

#[test]
fn test_1() {
    let input_rewards = vec![1, 2, 3, 1];
    let expected_output = 4;

    assert_eq!(Solution::rob(input_rewards), expected_output)
}
#[test]
fn test_2() {
    let input_rewards = vec![2, 7, 9, 3, 1];
    let expected_output = 12;

    assert_eq!(Solution::rob(input_rewards), expected_output)
}

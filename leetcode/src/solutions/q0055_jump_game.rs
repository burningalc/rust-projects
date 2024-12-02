/* LeetCode Problem Question 0055: Jump Game

You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.

https://leetcode.com/problems/jump-game/
*/

pub struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut memo = vec![0; nums.len()];
        memo[0] = nums[0];

        for (index, &num) in nums.iter().enumerate().skip(1) {
            memo[index] = std::cmp::max(num, memo[index - 1] - 1);
        }

        !memo.iter().take(memo.len() - 1).any(|e| *e == 0)
    }
}

#[test]
fn test_1() {
    let input = vec![2, 3, 1, 1, 4];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

#[test]
fn test_2() {
    let input = vec![3, 2, 1, 0, 4];
    let expected_output = false;

    assert_eq!(Solution::can_jump(input), expected_output)
}

#[test]
fn test_3() {
    let input = vec![0];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

#[test]
fn test_4() {
    let input = vec![2, 0, 0];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

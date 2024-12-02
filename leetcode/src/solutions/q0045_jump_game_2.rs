/* LeetCode Problem Question 0045: Jump Game 2

You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:
    0 <= j <= nums[i] and
    i + j < n

Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

https://leetcode.com/problems/jump-game-ii/
*/

pub struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // sliding window
        let mut right: usize = 0;
        let mut left: usize = 0;

        let mut jump: i32 = 0;

        let mut nums_iter = nums.iter();

        while left < nums.len() - 1 {
            // jump to the furthest among all steps inside the window
            let mut furthest = 0;
            for i in right..left + 1 {
                furthest = std::cmp::max(
                    furthest,
                    *nums_iter.next().unwrap_or(&0) - (left - i) as i32,
                );
            }
            // perform the jump
            right = left + 1;
            left += furthest as usize;
            jump += 1;
        }

        jump
    }
}

#[test]
fn test_1() {
    let input = vec![2, 3, 1, 1, 4];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_2() {
    let input = vec![2, 3, 0, 1, 4];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_3() {
    let input = vec![1, 2];
    let expected_output = 1;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_4() {
    let input = vec![2, 3, 1];
    let expected_output = 1;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_5() {
    let input = vec![1, 2, 3];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_6() {
    let input = vec![1, 1, 1, 1];
    let expected_output = 3;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_7() {
    let input = vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_8() {
    let input = vec![3, 4, 3, 2, 5, 4, 3];
    let expected_output = 3;

    assert_eq!(Solution::jump(input), expected_output)
}

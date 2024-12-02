/* LeetCode Problem Question 1822: Sign of the Product of an Array

Implement a function signFunc(x) that returns:
    1 if x is positive.
    -1 if x is negative.
    0 if x is equal to 0.

You are given an integer array nums. Let product be the product of all values in the array nums.

Return signFunc(product).

https://leetcode.com/problems/sign-of-the-product-of-an-array/
*/

pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for num in nums {
            if num == 0 {
                return 0;
            }
            if num < 0 {
                sign = -sign;
            }
        }
        sign
    }
}

#[test]
fn test_1() {
    let nums: Vec<i32> = vec![-1, -2, -3, -4, 3, 2, 1];

    let expected_output = 1;

    assert_eq!(Solution::array_sign(nums), expected_output);
}

#[test]
fn test_2() {
    let nums: Vec<i32> = vec![-1, 1, -1, 1, -1];

    let expected_output = -1;

    assert_eq!(Solution::array_sign(nums), expected_output);
}

#[test]
fn test_3() {
    let nums: Vec<i32> = vec![-1, -2, -3, -4, 3, 2, 1];

    let expected_output = 1;

    assert_eq!(Solution::array_sign(nums), expected_output);
}

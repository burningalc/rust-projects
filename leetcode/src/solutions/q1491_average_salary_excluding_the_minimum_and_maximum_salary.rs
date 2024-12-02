/* LeetCode Problem Question 1491: Average Salary Excluding the Minimum and Maximum Salary

You are given an array of unique integers salary where salary[i] is the salary of the ith employee.

Return the average salary of employees excluding the minimum and maximum salary. Answers within 10-5 of the actual answer will be accepted.

https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
*/

pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let max = *salary.iter().max().unwrap() as f64;
        let min = *salary.iter().min().unwrap() as f64;
        (salary.iter().sum::<i32>() as f64 - max - min) / (salary.len() - 2) as f64
    }

    pub fn average_single_pass(salary: Vec<i32>) -> f64 {
        let mut max = 999;
        let mut min = 10_000_000;
        let mut sum = 0;

        for s in &salary {
            sum += *s;

            if *s > max {
                max = *s;
            }

            if *s < min {
                min = *s;
            }
        }

        (sum - max - min) as f64 / (salary.len() - 2) as f64
    }
}

#[test]
fn test_1() {
    let input = vec![4000, 3000, 1000, 2000];
    let expected_output = 2500_f64;
    assert_eq!(Solution::average(input), expected_output);
}

#[test]
fn test_2() {
    let input = vec![1000, 2000, 3000];
    let expected_output = 2000_f64;
    assert_eq!(Solution::average(input), expected_output);
}

#[test]
fn test_single_pass() {
    let input = vec![4000, 3000, 1000, 2000];
    let expected_output = 2500_f64;
    assert_eq!(Solution::average_single_pass(input), expected_output);
}

/* LeetCode Problem Question 0150: Evaluate Reverse Polish Notation

You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

Evaluate the expression. Return an integer that represents the value of the expression.

Note that:
    The valid operators are '+', '-', '*', and '/'.
    Each operand may be an integer or another expression.
    The division between two integers always truncates toward zero.
    There will not be any division by zero.
    The input represents a valid arithmetic expression in a reverse polish notation.
    The answer and all the intermediate calculations can be represented in a 32-bit integer.


https://leetcode.com/problems/evaluate-reverse-polish-notation/
*/
pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut values = vec![];

        // decide if token is number or arithmetic operators
        for token in tokens.iter() {
            match token.as_str() {
                "+" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v1 + v2);
                }
                "-" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v2 - v1);
                }
                "*" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v1 * v2);
                }
                "/" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v2 / v1);
                }
                _ => values.push(token.parse::<i32>().unwrap()),
            }
        }

        values.pop().unwrap()
    }
}

#[test]
fn test_1() {
    let test_case = ["2", "1", "+", "3", "*"];
    let expected = 9;

    assert_eq!(
        Solution::eval_rpn(test_case.iter().map(|s| s.to_string()).collect()),
        expected
    );
}

#[test]
fn test_2() {
    let test_case = ["4", "13", "5", "/", "+"];
    let expected = 6;

    assert_eq!(
        Solution::eval_rpn(test_case.iter().map(|s| s.to_string()).collect()),
        expected
    );
}

#[test]
fn test_3() {
    let test_case = [
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ];
    let expected = 22;

    assert_eq!(
        Solution::eval_rpn(test_case.iter().map(|s| s.to_string()).collect()),
        expected
    );
}

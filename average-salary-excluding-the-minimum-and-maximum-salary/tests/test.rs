use average_salary_excluding_the_minimum_and_maximum_salary::Solution;
use time_test::time_test;

#[test]
fn test_1() {
    let input = vec![4000, 3000, 1000, 2000];
    let expected_output = 2500_f64;
    time_test!("test_1");
    assert_eq!(Solution::average(input), expected_output);
}

#[test]
fn test_2() {
    let input = vec![1000, 2000, 3000];
    let expected_output = 2000_f64;
    time_test!("test_2");
    assert_eq!(Solution::average(input), expected_output);
}

#[test]
fn test_single_pass() {
    let input = vec![4000, 3000, 1000, 2000];
    let expected_output = 2500_f64;
    time_test!("test_3");
    assert_eq!(Solution::average_single_pass(input), expected_output);
}

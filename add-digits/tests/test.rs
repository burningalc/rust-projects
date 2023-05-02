use add_digits::Solution;

#[test]
fn test_1() {
    let input = 38;
    let expected_output = 2;

    assert_eq!(Solution::add_digits(input), expected_output)
}

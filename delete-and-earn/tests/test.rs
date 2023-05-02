use delete_and_earn::Solution;

#[test]
fn test_1() {
    let input = vec![3, 4, 2];
    let expected_output = 6;

    assert_eq!(expected_output, Solution::delete_and_earn(input));
}

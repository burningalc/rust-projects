use house_robber::Solution;

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

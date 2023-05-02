use house_robber::Solution;

#[test]
fn test_1() {
    let input_rewards = vec![2, 3, 2];
    let expected_output = 3;

    assert_eq!(Solution::rob(input_rewards), expected_output)
}
#[test]
fn test_2() {
    let input_rewards = vec![1, 2, 3, 1];
    let expected_output = 4;

    assert_eq!(Solution::rob(input_rewards), expected_output)
}

#[test]
fn test_3() {
    let input_rewards = vec![1, 2, 3];
    let expected_output = 3;

    assert_eq!(Solution::rob(input_rewards), expected_output)
}

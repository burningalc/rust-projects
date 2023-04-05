use jump_game::Solution;

#[test]
fn test_1() {
    let input = vec![2, 3, 1, 1, 4];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

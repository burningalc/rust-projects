use jump_game::Solution;

#[test]
fn test_1() {
    let input = vec![2, 3, 1, 1, 4];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

#[test]
fn test_2() {
    let input = vec![3, 2, 1, 0, 4];
    let expected_output = false;

    assert_eq!(Solution::can_jump(input), expected_output)
}

#[test]
fn test_3() {
    let input = vec![0];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

#[test]
fn test_4() {
    let input = vec![2, 0, 0];
    let expected_output = true;

    assert_eq!(Solution::can_jump(input), expected_output)
}

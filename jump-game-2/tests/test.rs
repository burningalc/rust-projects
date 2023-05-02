use jump_game_2::Solution;

#[test]
fn test_1() {
    let input = vec![2, 3, 1, 1, 4];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_2() {
    let input = vec![2, 3, 0, 1, 4];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_3() {
    let input = vec![1, 2];
    let expected_output = 1;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_4() {
    let input = vec![2, 3, 1];
    let expected_output = 1;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_5() {
    let input = vec![1, 2, 3];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_6() {
    let input = vec![1, 1, 1, 1];
    let expected_output = 3;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_7() {
    let input = vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3];
    let expected_output = 2;

    assert_eq!(Solution::jump(input), expected_output)
}

#[test]
fn test_8() {
    let input = vec![3, 4, 3, 2, 5, 4, 3];
    let expected_output = 3;

    assert_eq!(Solution::jump(input), expected_output)
}

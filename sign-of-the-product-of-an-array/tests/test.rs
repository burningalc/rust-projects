use sign_of_the_product_of_an_array::Solution;

#[test]
fn test_1() {
    let nums: Vec<i32> = vec![-1, -2, -3, -4, 3, 2, 1];

    let expected_output = 1;

    assert_eq!(Solution::array_sign(nums), expected_output);
}

#[test]
fn test_2() {
    let nums: Vec<i32> = vec![-1, 1, -1, 1, -1];

    let expected_output = -1;

    assert_eq!(Solution::array_sign(nums), expected_output);
}

#[test]
fn test_3() {
    let nums: Vec<i32> = vec![-1, -2, -3, -4, 3, 2, 1];

    let expected_output = 1;

    assert_eq!(Solution::array_sign(nums), expected_output);
}

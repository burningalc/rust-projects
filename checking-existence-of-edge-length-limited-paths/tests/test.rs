use checking_existence_of_edge_length_limited_paths::Solution;

#[test]
fn test_1() {
    let n = 3;
    let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
    let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];

    let expected_output = vec![false, true];

    assert_eq!(
        Solution::distance_limited_paths_exist(n, edge_list, queries),
        expected_output
    );
}

#[test]
fn test_2() {
    let n = 5;
    let edge_list = vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]];
    let queries = vec![vec![0, 4, 14], vec![1, 4, 13]];

    let expected_output = vec![true, false];

    assert_eq!(
        Solution::distance_limited_paths_exist(n, edge_list, queries),
        expected_output
    );
}

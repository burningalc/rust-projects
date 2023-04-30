use remove_max_number_of_edges_to_keep_graph_fully_traversable::Solution;

#[test]
fn test_1() {
    let n = 4;
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4],
    ];

    let expected_output = 2;

    assert_eq!(Solution::max_num_edges_to_remove(n, edges), expected_output);
}

#[test]
fn test_2() {
    let n = 4;
    let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];

    let expected_output = 0;

    assert_eq!(Solution::max_num_edges_to_remove(n, edges), expected_output);
}

#[test]
fn test_3() {
    let n = 4;
    let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];

    let expected_output = -1;

    assert_eq!(Solution::max_num_edges_to_remove(n, edges), expected_output);
}

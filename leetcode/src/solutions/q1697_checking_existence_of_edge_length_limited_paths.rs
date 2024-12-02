/* LeetCode Problem 1697: Checking Existence of Edge Length Limited Paths

An undirected graph of n nodes is defined by edgeList, where edgeList[i] = [ui, vi, disi] denotes an edge between nodes ui and vi with distance disi. Note that there may be multiple edges between two nodes.

Given an array queries, where queries[j] = [pj, qj, limitj], your task is to determine for each queries[j] whether there is a path between pj and qj such that each edge on the path has a distance strictly less than limitj .

Return a boolean array answer, where answer.length == queries.length and the jth value of answer is true if there is a path for queries[j] is true, and false otherwise.

https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/
*/

pub struct Solution;

impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut groups: Vec<usize> = (0..n as usize).collect();
        let mut result: Vec<bool> = vec![false; queries.len()];

        // sort both edge_list and queries
        let mut sorted_edge_list = edge_list;
        sorted_edge_list.sort_unstable_by_key(|e| e[2]);
        let mut sorted_queries_index: Vec<usize> = (0..queries.len()).collect();
        sorted_queries_index.sort_unstable_by_key(|&index| queries[index][2]);

        let mut sorted_edge_list = sorted_edge_list.iter().peekable();
        for index in sorted_queries_index {
            let limit = queries[index][2];
            // add all the edges with less than the query size
            while let Some(edge) = sorted_edge_list.peek() {
                if edge[2] < limit {
                    Self::union_root(&mut groups, edge[0] as usize, edge[1] as usize);
                    sorted_edge_list.next();
                } else {
                    break;
                }
            }

            // check if the node is connected
            if Self::is_node_connected(
                &mut groups,
                queries[index][0] as usize,
                queries[index][1] as usize,
            ) {
                result[index] = true;
            }
        }

        result
    }

    fn union_root(groups: &mut [usize], node_a: usize, node_b: usize) {
        let root_a = Self::find_root(groups, node_a);
        let root_b = Self::find_root(groups, node_b);

        if root_a != root_b {
            groups[root_a] = root_b;
        }
    }

    fn find_root(groups: &mut [usize], node: usize) -> usize {
        if groups[node] == node {
            return node;
        }

        let root = Self::find_root(groups, groups[node]);

        groups[node] = root;

        root
    }

    fn is_node_connected(groups: &mut [usize], node_a: usize, node_b: usize) -> bool {
        Self::find_root(groups, node_a) == Self::find_root(groups, node_b)
    }
}

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

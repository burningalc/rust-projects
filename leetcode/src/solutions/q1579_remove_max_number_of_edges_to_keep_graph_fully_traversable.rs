/* LeetCode Problem Question 1579: Remove Max Number of Edges to Keep Graph Fully Traversable

Alice and Bob have an undirected graph of n nodes and three types of edges:
    Type 1: Can be traversed by Alice only.
    Type 2: Can be traversed by Bob only.
    Type 3: Can be traversed by both Alice and Bob.

Given an array edges where edges[i] = [typei, ui, vi] represents a bidirectional edge of type typei between nodes ui and vi, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.

Return the maximum number of edges you can remove, or return -1 if Alice and Bob cannot fully traverse the graph.
*/

use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut alice_groups: Vec<usize> = (0..n as usize).collect();
        let mut bob_groups: Vec<usize> = (0..n as usize).collect();

        let mut removed_edge = 0;

        // sort the edges according to their type
        let mut sorted_edges = edges;
        sorted_edges.sort_unstable_by_key(|f| Reverse(f[0]));

        for edge in sorted_edges {
            let node_a = edge[1] as usize - 1;
            let node_b = edge[2] as usize - 1;
            match edge[0] {
                3 => {
                    let added_alice = Self::union_root(&mut alice_groups, node_a, node_b);
                    let added_bob = Self::union_root(&mut bob_groups, node_a, node_b);
                    if !added_alice && !added_bob {
                        removed_edge += 1;
                    }
                }
                2 => {
                    if !Self::union_root(&mut alice_groups, node_a, node_b) {
                        removed_edge += 1;
                    }
                }
                1 => {
                    if !Self::union_root(&mut bob_groups, node_a, node_b) {
                        removed_edge += 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        // check if both of the groups are individually connected
        if !Self::group_connected(&mut alice_groups) || !Self::group_connected(&mut bob_groups) {
            return -1;
        }

        removed_edge
    }

    fn union_root(groups: &mut [usize], node_a: usize, node_b: usize) -> bool {
        let root_a = Self::find_root(groups, node_a);
        let root_b = Self::find_root(groups, node_b);

        // check if the two node share the same root
        if root_a == root_b {
            return false;
        }

        // union the two root
        groups[root_a] = root_b;

        true
    }

    fn find_root(groups: &mut [usize], node: usize) -> usize {
        if groups[node] == node {
            return node;
        }
        let root = Self::find_root(groups, groups[node]);
        groups[node] = root;
        root
    }

    fn group_connected(groups: &mut [usize]) -> bool {
        let root = Self::find_root(groups, 0);

        for i in 1..groups.len() {
            if Self::find_root(groups, i) != root {
                return false;
            }
        }
        true
    }
}

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

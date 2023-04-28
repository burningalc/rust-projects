use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut groups: Vec<usize> = (0..strs.len()).collect();

        // iterate the strs and assign each to their group
        for i in 0..strs.len() {
            // compare all strs before index i
            for j in 0..i {
                if Self::is_similar(&strs[i], &strs[j]) {
                    Self::union_root(&mut groups, i, j)
                }
            }
        }

        // report how many groups there are,
        // by use a hash set to filter duplicates
        let unique_groups = (0..strs.len())
            .map(|i| Self::find_root(&groups, i))
            .collect::<HashSet<usize>>();
        unique_groups.len() as i32
    }

    pub fn is_similar(a: &String, b: &String) -> bool {
        if a == b {
            return true;
        }

        let mut misplaced_count = 0;
        for (char_a, char_b) in a.chars().zip(b.chars()) {
            if char_a != char_b {
                misplaced_count += 1;
            }
            if misplaced_count == 3 {
                return false;
            }
        }
        true
    }

    pub fn union_root(groups: &mut [usize], i: usize, j: usize) {
        // connect i and j in the disjoint set by union-find algorithm
        let root_i = Self::find_root(groups, i);
        let root_j = Self::find_root(groups, j);
        groups[root_i] = root_j;
    }

    pub fn find_root(groups: &[usize], node: usize) -> usize {
        let mut current_node = node;
        loop {
            if groups[current_node] == current_node {
                return current_node;
            } else {
                current_node = groups[current_node];
            }
        }
    }
}

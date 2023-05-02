#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut memo: HashMap<usize, i32> = HashMap::new();
        Self::get_cost(root, &mut memo)
    }

    pub fn get_cost(node: Option<Rc<RefCell<TreeNode>>>, memo: &mut HashMap<usize, i32>) -> i32 {
        if node.is_none() {
            return 0;
        }
        let binding = node.unwrap();
        let node = binding.borrow_mut();

        // check if node has been visited
        let node_id = binding.as_ptr() as usize;
        if let Some(&val) = memo.get(&node_id) {
            return val;
        }

        let mut node_cost = node.val;
        let mut node_children_cost = 0;
        if node.left.is_some() {
            let left_node = node.left.clone().unwrap();
            node_cost += Self::get_cost(left_node.borrow().left.clone(), memo);
            node_cost += Self::get_cost(left_node.borrow().right.clone(), memo);

            node_children_cost += Self::get_cost(Some(left_node), memo);
        }

        if node.right.is_some() {
            let right_node = node.right.clone().unwrap();
            node_cost += Self::get_cost(right_node.borrow().left.clone(), memo);
            node_cost += Self::get_cost(right_node.borrow().right.clone(), memo);

            node_children_cost += Self::get_cost(Some(right_node), memo);
        }

        let max = std::cmp::max(node_cost, node_children_cost);
        memo.insert(node_id, max);
        max
    }
}

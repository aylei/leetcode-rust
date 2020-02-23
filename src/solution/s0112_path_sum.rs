/**
 * [112] Path Sum
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \      \
 * 7    2      1
 *
 *
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/path-sum/
// discuss: https://leetcode.com/problems/path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut deq = VecDeque::new();
        deq.push_back((0, root.unwrap().clone()));
        while !deq.is_empty() {
            if let Some((acc, node)) = deq.pop_front() {
                let acc = acc + node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if acc == sum {
                        return true;
                    }
                } else {
                    if node.borrow().left.is_some() {
                        deq.push_back((acc, node.borrow().left.as_ref().unwrap().clone()))
                    };
                    if node.borrow().right.is_some() {
                        deq.push_back((acc, node.borrow().right.as_ref().unwrap().clone()))
                    };
                }
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(
            Solution::has_path_sum(
                tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22
            ),
            true
        );
    }
}

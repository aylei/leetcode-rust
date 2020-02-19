/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
 *
 *
 * For example:<br />
 * Given binary tree [3,9,20,null,null,15,7],<br />
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 *
 * return its bottom-up level order traversal as:<br />
 *
 * [
 *   [15,7],
 *   [9,20],
 *   [3]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
// discuss: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut current_level = 0;
        if root.is_none() {
            return res;
        }
        let mut deq = VecDeque::new();
        deq.push_back((0, root.clone()));
        let mut vec = Vec::new();
        while !deq.is_empty() {
            if let Some((level, Some(node))) = deq.pop_front() {
                deq.push_back((level + 1, node.borrow().left.clone()));
                deq.push_back((level + 1, node.borrow().right.clone()));
                if level > current_level {
                    res.push(vec);
                    vec = Vec::new();
                    current_level = level;
                }
                vec.push(node.borrow().val);
            }
        }
        if !vec.is_empty() {
            res.push(vec)
        }
        res.reverse();
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_107() {
        assert_eq!(
            Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![15, 7], vec![9, 20], vec![3],]
        );
    }
}

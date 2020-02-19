/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
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
 * return its zigzag level order traversal as:<br />
 *
 * [
 *   [3],
 *   [20,9],
 *   [15,7]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
                    if current_level % 2 == 1 {
                        vec.reverse();
                    }
                    res.push(vec);
                    vec = Vec::new();
                    current_level = level;
                }
                vec.push(node.borrow().val);
            }
        }
        if !vec.is_empty() {
            if current_level % 2 == 1 {
                vec.reverse();
            }
            res.push(vec)
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_103() {
        assert_eq!(
            Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}

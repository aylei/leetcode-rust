/**
 * [113] Path Sum II
 *
 * Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.
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
 *  /  \    / \
 * 7    2  5   1
 *
 *
 * Return:
 *
 *
 * [
 *    [5,4,11,2],
 *    [5,8,4,5]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/path-sum-ii/
// discuss: https://leetcode.com/problems/path-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        let mut deq = VecDeque::new();
        deq.push_back((0, Vec::new(), root.clone()));
        while !deq.is_empty() {
            if let Some((acc, mut vec, Some(node))) = deq.pop_front() {
                let acc = acc + node.borrow().val;
                vec.push(node.borrow().val);
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if acc == sum {
                        res.push(vec);
                    }
                } else {
                    if node.borrow().left.is_some() {
                        deq.push_back((acc, vec.clone(), node.borrow().left.clone()));
                    }
                    if node.borrow().right.is_some() {
                        deq.push_back((acc, vec.clone(), node.borrow().right.clone()));
                    }
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_113() {
        assert_eq!(
            Solution::path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        )
    }
}

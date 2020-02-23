/**
 * [199] Binary Tree Right Side View
 *
 * Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 *
 * Example:
 *
 *
 * Input: [1,2,3,null,5,null,4]
 * Output: [1, 3, 4]
 * Explanation:
 *
 *    1            <---
 *  /   \
 * 2     3         <---
 *  \     \
 *   5     4       <---
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-right-side-view/
// discuss: https://leetcode.com/problems/binary-tree-right-side-view/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut current_level = 0;
        if root.is_none() {
            return res;
        }
        let mut deq = VecDeque::new();
        deq.push_back((0, root.clone()));
        res.push(root.as_ref().unwrap().borrow().val);
        while !deq.is_empty() {
            if let Some((level, Some(node))) = deq.pop_front() {
                deq.push_back((level + 1, node.borrow().right.clone()));
                deq.push_back((level + 1, node.borrow().left.clone()));
                if level > current_level {
                    res.push(node.borrow().val);
                    current_level = level;
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
    fn test_199() {
        assert_eq!(
            Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4]),
            vec![1, 3, 4]
        );
    }
}

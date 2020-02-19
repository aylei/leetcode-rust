/**
 * [226] Invert Binary Tree
 *
 * Invert a binary tree.
 *
 * Example:
 *
 * Input:
 *
 *
 *      4
 *    /   \
 *   2     7
 *  / \   / \
 * 1   3 6   9
 *
 * Output:
 *
 *
 *      4
 *    /   \
 *   7     2
 *  / \   / \
 * 9   6 3   1
 *
 * Trivia:<br />
 * This problem was inspired by <a href="https://twitter.com/mxcl/status/608682016205344768" target="_blank">this original tweet</a> by <a href="https://twitter.com/mxcl" target="_blank">Max Howell</a>:
 *
 * <blockquote>Google: 90% of our engineers use the software you wrote (Homebrew), but you can&rsquo;t invert a binary tree on a whiteboard so f*** off.</blockquote>
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/invert-binary-tree/
// discuss: https://leetcode.com/problems/invert-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            Solution::invert_tree(node.borrow().right.clone());
            Solution::invert_tree(node.borrow().left.clone());
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_226() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}

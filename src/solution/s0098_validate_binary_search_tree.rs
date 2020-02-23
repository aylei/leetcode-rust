/**
 * [98] Validate Binary Search Tree
 *
 * Given a binary tree, determine if it is a valid binary search tree (BST).
 *
 * Assume a BST is defined as follows:
 *
 *
 * 	The left subtree of a node contains only nodes with keys less than the node's key.
 * 	The right subtree of a node contains only nodes with keys greater than the node's key.
 * 	Both the left and right subtrees must also be binary search trees.
 *
 *
 * Example 1:
 *
 *
 * Input:
 *     2
 *    / \
 *   1   3
 * Output: true
 *
 *
 * Example 2:
 *
 *
 *     5
 *    / \
 *   1   4
 *      / \
 *     3   6
 * Output: false
 * Explanation: The input is: [5,1,4,null,null,3,6]. The root node's value
 *              is 5 but its right child's value is 4.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/validate-binary-search-tree/
// discuss: https://leetcode.com/problems/validate-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut vec = vec![];
        Solution::preorder_traverse(root.as_ref(), &mut vec)
    }

    fn preorder_traverse(
        root: Option<&Rc<RefCell<TreeNode>>>,
        formers: &mut Vec<(i32, i32)>,
    ) -> bool {
        if let Some(node) = root {
            let root_val = root.as_ref().unwrap().borrow().val;
            for former in formers.iter() {
                if (former.0 < 0 && root_val >= former.1) || (former.0 > 0 && root_val <= former.1)
                {
                    return false;
                }
            }
            let mut to_right = formers.clone();
            formers.push((-1, root_val));
            to_right.push((1, root_val));
            Solution::preorder_traverse(node.borrow().left.as_ref(), formers)
                && Solution::preorder_traverse(node.borrow().right.as_ref(), &mut to_right)
        } else {
            true
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_98() {
        assert_eq!(
            Solution::is_valid_bst(tree![5, 1, 4, null, null, 3, 6]),
            false
        );
        assert_eq!(Solution::is_valid_bst(tree![2, 1, 3]), true);
        assert_eq!(
            Solution::is_valid_bst(tree![10, 5, 15, null, null, 6, 20]),
            false
        );
    }
}

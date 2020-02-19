/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * Given inorder and postorder traversal of a tree, construct the binary tree.
 *
 * Note:<br />
 * You may assume that duplicates do not exist in the tree.
 *
 * For example, given
 *
 *
 * inorder = [9,3,15,20,7]
 * postorder = [9,15,7,20,3]
 *
 * Return the following binary tree:
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree_helper(&postorder[..], &inorder[..])
    }

    fn build_tree_helper(postorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        let root_idx = inorder
            .iter()
            .position(|v| v == postorder.last().unwrap())
            .unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: *postorder.last().unwrap(),
            left: Solution::build_tree_helper(&postorder[0..root_idx], &inorder[0..root_idx]),
            right: Solution::build_tree_helper(
                &postorder[root_idx..postorder.len() - 1],
                &inorder[root_idx + 1..],
            ),
        })))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_106() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        assert_eq!(
            Solution::build_tree(vec![3, 20, 7], vec![7, 20, 3]),
            tree![3, null, 20, null, 7]
        );
        assert_eq!(Solution::build_tree(vec![], vec![]), tree![]);
    }
}

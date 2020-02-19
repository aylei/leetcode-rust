/**
 * [94] Binary Tree Inorder Traversal
 *
 * Given a binary tree, return the inorder traversal of its nodes' values.
 *
 * Example:
 *
 *
 * Input: [1,null,2,3]
 *    1
 *     \
 *      2
 *     /
 *    3
 *
 * Output: [1,3,2]
 *
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use crate::util::tree::{to_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Solution::inorder_traverse(root.as_ref(), &mut (|v| res.push(v)));
        res
    }

    fn inorder_traverse<F: FnMut(i32)>(root: Option<&Rc<RefCell<TreeNode>>>, consumer: &mut F) {
        if let Some(node) = root {
            Solution::inorder_traverse(node.borrow().left.as_ref(), consumer);
            consumer(root.as_ref().unwrap().borrow().val);
            Solution::inorder_traverse(node.borrow().right.as_ref(), consumer);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_94() {
        assert_eq!(
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(
            Solution::inorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![4, 2, 5, 1, 6, 3, 7]
        );
    }
}

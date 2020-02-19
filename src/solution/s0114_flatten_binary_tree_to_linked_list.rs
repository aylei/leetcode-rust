/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given a binary tree, flatten it to a linked list in-place.
 *
 * For example, given the following tree:
 *
 *
 *     1
 *    / \
 *   2   5
 *  / \   \
 * 3   4   6
 *
 *
 * The flattened tree should look like:
 *
 *
 * 1
 *  \
 *   2
 *    \
 *     3
 *      \
 *       4
 *        \
 *         5
 *          \
 *           6
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
// discuss: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Solution::flatten_helper(root.clone());
    }

    fn flatten_helper(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let tail = Solution::flatten_helper(node.borrow().left.clone());
            Solution::flatten_helper(node.borrow_mut().right.clone());
            let mut right = node.borrow().right.clone();
            let mut ptr = node.clone();
            if let Some(tail) = tail {
                let head = node.borrow_mut().left.take();
                node.borrow_mut().right = head;
                tail.borrow_mut().right = right.clone();
                ptr = tail.clone();
            }
            while let Some(next) = ptr.clone().borrow().right.clone() {
                ptr = next
            }
            Some(ptr)
        } else {
            None
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_114() {
        let mut tree = tree![1, 2, 5, 3, 4, null, 6];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);

        let mut tree = tree![1, 2, null, 3];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3]);

        let mut tree = tree![1, null, 2, 3];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3]);
    }
}

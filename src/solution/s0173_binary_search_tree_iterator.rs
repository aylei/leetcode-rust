/**
 * [173] Binary Search Tree Iterator
 *
 * Implement an iterator over a binary search tree (BST). Your iterator will be initialized with the root node of a BST.
 *
 * Calling next() will return the next smallest number in the BST.
 *
 *
 *
 *
 *
 *
 * Example:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/25/bst-tree.png" style="width: 189px; height: 178px;" />
 *
 *
 * BSTIterator iterator = new BSTIterator(root);
 * iterator.next();    // return 3
 * iterator.next();    // return 7
 * iterator.hasNext(); // return true
 * iterator.next();    // return 9
 * iterator.hasNext(); // return true
 * iterator.next();    // return 15
 * iterator.hasNext(); // return true
 * iterator.next();    // return 20
 * iterator.hasNext(); // return false
 *
 *
 *
 *
 * Note:
 *
 *
 * 	next() and hasNext() should run in average O(1) time and uses O(h) memory, where h is the height of the tree.
 * 	You may assume that next() call will always be valid, that is, there will be at least a next smallest number in the BST when next() is called.
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

// problem: https://leetcode.com/problems/binary-search-tree-iterator/
// discuss: https://leetcode.com/problems/binary-search-tree-iterator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
非递归中序遍历
*/
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut node = root;
        let mut stack = Vec::new();
        while let Some(inner) = node.clone() {
            stack.push(inner.clone());
            node = node.unwrap().borrow().left.clone();
        }
        BSTIterator { stack: stack }
    }

    /** @return the next smallest number */
    pub fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let res = node.borrow().val;
        let mut next = node.borrow().right.clone();
        while let Some(inner) = next.clone() {
            self.stack.push(inner.clone());
            next = next.unwrap().borrow().left.clone();
        }
        res
    }

    /** @return whether we have a next smallest number */
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_173() {
        let mut iterator = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
        assert_eq!(iterator.next(), 3); // return 3
        assert_eq!(iterator.next(), 7); // return 7
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 9); // return 9
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 15); // return 15
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 20); // return 20
        assert_eq!(iterator.has_next(), false); // return false
    }
}

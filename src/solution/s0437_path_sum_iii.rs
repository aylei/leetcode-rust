/**
 * [437] Path Sum III
 *
 * You are given a binary tree in which each node contains an integer value.
 *
 * Find the number of paths that sum to a given value.
 *
 * The path does not need to start or end at the root or a leaf, but it must go downwards
 * (traveling only from parent nodes to child nodes).
 *
 * The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.
 *
 * Example:
 *
 * root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8
 *
 *       10
 *      /  \
 *     5   -3
 *    / \    \
 *   3   2   11
 *  / \   \
 * 3  -2   1
 *
 * Return 3. The paths that sum to 8 are:
 *
 * 1.  5 -> 3
 * 2.  5 -> 2 -> 1
 * 3. -3 -> 11
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        return Self::helper(&root, sum);
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let node = root.as_ref().unwrap();
        let val = node.borrow().val;
        let left = &node.borrow().left;
        let right = &node.borrow().right;
        return Self::helper_include(root, sum)
            + Self::helper(left, sum)
            + Self::helper(right, sum);
    }

    pub fn helper_include(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let node = root.as_ref().unwrap();
        let val = node.borrow().val;
        let left = &node.borrow().left;
        let right = &node.borrow().right;
        let curr_res = if sum == val { 1 } else { 0 };
        return Self::helper_include(left, sum - val)
            + Self::helper_include(right, sum - val)
            + curr_res;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_437() {}
}

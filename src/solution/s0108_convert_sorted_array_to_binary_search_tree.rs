/**
 * [108] Convert Sorted Array to Binary Search Tree
 *
 * Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
 *
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 *
 * Example:
 *
 *
 * Given the sorted array: [-10,-3,0,5,9],
 *
 * One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
 *
 *       0
 *      / \
 *    -3   9
 *    /   /
 *  -10  5
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst_helper(&nums[..])
    }

    fn bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[nums.len() / 2],
            left: Solution::bst_helper(&nums[0..(nums.len() / 2)]),
            right: Solution::bst_helper(&nums[(nums.len() / 2 + 1)..]),
        })))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
        assert_eq!(Solution::sorted_array_to_bst(vec![]), tree![]);
    }
}

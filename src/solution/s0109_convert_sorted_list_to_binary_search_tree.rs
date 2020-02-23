/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
 *
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 *
 * Example:
 *
 *
 * Given the sorted linked list: [-10,-3,0,5,9],
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
use crate::util::linked_list::{to_list, ListNode};
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut arr = Vec::new();
        let mut head = head;
        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }
        Solution::bst_helper(&arr[..])
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
    fn test_109() {
        assert_eq!(
            Solution::sorted_list_to_bst(linked![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
    }
}

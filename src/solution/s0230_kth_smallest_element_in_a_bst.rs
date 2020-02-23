/**
 * [230] Kth Smallest Element in a BST
 *
 * Given a binary search tree, write a function kthSmallest to find the kth smallest element in it.
 *
 * Note: <br />
 * You may assume k is always valid, 1 &le; k &le; BST's total elements.
 *
 * Example 1:
 *
 *
 * Input: root = [3,1,4,null,2], k = 1
 *    3
 *   / \
 *  1   4
 *   \
 *    2
 * Output: 1
 *
 * Example 2:
 *
 *
 * Input: root = [5,3,6,2,4,null,null,1], k = 3
 *        5
 *       / \
 *      3   6
 *     / \
 *    2   4
 *   /
 *  1
 * Output: 3
 *
 *
 * Follow up:<br />
 * What if the BST is modified (insert/delete operations) often and you need to find the kth smallest frequently? How would you optimize the kthSmallest routine?
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// discuss: https://leetcode.com/problems/kth-smallest-element-in-a-bst/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        Solution::helper(root, k, &mut res);
        res
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, k: i32, res: &mut i32) -> i32 {
        if k <= 0 {
            return 0;
        }
        if let Some(node) = root {
            let left = Solution::helper(node.borrow().left.clone(), k, res);
            if left == 1 {
                *res = node.borrow().val;
            }
            let right = Solution::helper(node.borrow().right.clone(), left - 1, res);
            right
        } else {
            k
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_230() {
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 1), 1);
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 2), 2);
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 3), 3);
    }
}

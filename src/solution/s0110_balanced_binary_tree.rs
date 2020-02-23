/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
 *
 * For this problem, a height-balanced binary tree is defined as:
 *
 * <blockquote>
 * a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 * </blockquote>
 *
 * Example 1:
 *
 * Given the following tree [3,9,20,null,null,15,7]:
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 * Return true.<br />
 * <br />
 * Example 2:
 *
 * Given the following tree [1,2,2,3,3,null,null,4,4]:
 *
 *
 *        1
 *       / \
 *      2   2
 *     / \
 *    3   3
 *   / \
 *  4   4
 *
 *
 * Return false.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/balanced-binary-tree/
// discuss: https://leetcode.com/problems/balanced-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::balanced_helper(root.as_ref()).is_some()
    }

    fn balanced_helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let pair = (
                Solution::balanced_helper(node.borrow().left.as_ref()),
                Solution::balanced_helper(node.borrow().right.as_ref()),
            );
            match pair {
                (Some(left), Some(right)) => {
                    if i32::abs(left - right) < 2 {
                        return Some(i32::max(left, right) + 1);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        } else {
            Some(0)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        assert_eq!(Solution::is_balanced(tree![]), true);
        assert_eq!(
            Solution::is_balanced(tree![3, 9, 20, null, null, 15, 7]),
            true
        );
        assert_eq!(
            Solution::is_balanced(tree![1, 2, 2, 3, 3, null, null, 4, 4]),
            false
        );
    }
}

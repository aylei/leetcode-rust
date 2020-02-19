/**
 * [100] Same Tree
 *
 * Given two binary trees, write a function to check if they are the same or not.
 *
 * Two binary trees are considered the same if they are structurally identical and the nodes have the same value.
 *
 * Example 1:
 *
 *
 * Input:     1         1
 *           / \       / \
 *          2   3     2   3
 *
 *         [1,2,3],   [1,2,3]
 *
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input:     1         1
 *           /           \
 *          2             2
 *
 *         [1,2],     [1,null,2]
 *
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input:     1         1
 *           / \       / \
 *          2   1     1   2
 *
 *         [1,2,1],   [1,1,2]
 *
 * Output: false
 *
 *
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};
// problem: https://leetcode.com/problems/same-tree/
// discuss: https://leetcode.com/problems/same-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100() {
        assert_eq!(
            Solution::is_same_tree(tree![1, 2, 3, 4, null, 5], tree![1, 2, 3, 4, null, 5]),
            true
        )
    }
}

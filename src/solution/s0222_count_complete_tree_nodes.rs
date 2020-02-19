/**
 * [222] Count Complete Tree Nodes
 *
 * Given a complete binary tree, count the number of nodes.
 *
 * Note:
 *
 * <u>Definition of a complete binary tree from <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>:</u><br />
 * In a complete binary tree every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 *
 * Example:
 *
 *
 * Input:
 *     1
 *    / \
 *   2   3
 *  / \  /
 * 4  5 6
 *
 * Output: 6
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/count-complete-tree-nodes/
// discuss: https://leetcode.com/problems/count-complete-tree-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 0. get the hight of full nodes
        let mut height = 0;
        let mut curr = root.clone();
        let mut result = 0;
        while let Some(inner) = curr {
            height += 1;
            result += 2_i32.pow(height - 1);
            curr = inner.borrow().right.clone();
        }
        if height == 0 {
            return result;
        }
        // 1. 'binary search' to find the node number of the lowest level
        // the lowest level may have 0~2^H-1 node
        let mut curr_root = root.clone();
        while height > 1 {
            // see if left tree is full
            let mut node = curr_root.clone().unwrap().borrow().left.clone();
            let mut level = 2;
            while level < height {
                node = node.unwrap().borrow().right.clone();
                level += 1;
            }
            if node.unwrap().borrow().right.is_some() {
                curr_root = curr_root.unwrap().borrow().right.clone();
                result += 2_i32.pow(height - 1);
            } else {
                curr_root = curr_root.unwrap().borrow().left.clone();
            }
            height -= 1;
        }
        if curr_root.as_ref().unwrap().borrow().left.is_some() {
            result += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_222() {
        assert_eq!(Solution::count_nodes(tree![1, 1, 1, 1, 1, 1, 1]), 7);
        assert_eq!(Solution::count_nodes(tree![]), 0);
        assert_eq!(Solution::count_nodes(tree![1, 1]), 2);
        assert_eq!(Solution::count_nodes(tree![1]), 1);
        assert_eq!(Solution::count_nodes(tree![1, 1, 1]), 3);
    }
}

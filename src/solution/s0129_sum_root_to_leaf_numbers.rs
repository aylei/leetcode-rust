/**
 * [129] Sum Root to Leaf Numbers
 *
 * Given a binary tree containing digits from 0-9 only, each root-to-leaf path could represent a number.
 *
 * An example is the root-to-leaf path 1->2->3 which represents the number 123.
 *
 * Find the total sum of all root-to-leaf numbers.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 *
 * Input: [1,2,3]
 *     1
 *    / \
 *   2   3
 * Output: 25
 * Explanation:
 * The root-to-leaf path 1->2 represents the number 12.
 * The root-to-leaf path 1->3 represents the number 13.
 * Therefore, sum = 12 + 13 = 25.
 *
 * Example 2:
 *
 *
 * Input: [4,9,0,5,1]
 *     4
 *    / \
 *   9   0
 *  / \
 * 5   1
 * Output: 1026
 * Explanation:
 * The root-to-leaf path 4->9->5 represents the number 495.
 * The root-to-leaf path 4->9->1 represents the number 491.
 * The root-to-leaf path 4->0 represents the number 40.
 * Therefore, sum = 495 + 491 + 40 = 1026.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/sum-root-to-leaf-numbers/
// discuss: https://leetcode.com/problems/sum-root-to-leaf-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        if root.is_none() {
            return sum;
        }
        let mut deq = VecDeque::new();
        deq.push_back((root.clone(), 0));
        while let Some(item) = deq.pop_front() {
            if let (Some(node), acc) = item {
                let acc = acc * 10 + node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    sum += acc;
                    continue;
                }
                deq.push_back((node.borrow().left.clone(), acc));
                deq.push_back((node.borrow().right.clone(), acc));
            }
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_129() {
        assert_eq!(Solution::sum_numbers(tree![1, 2, 3]), 25);
        assert_eq!(Solution::sum_numbers(tree![4, 9, 0, 5, 1]), 1026);
    }
}

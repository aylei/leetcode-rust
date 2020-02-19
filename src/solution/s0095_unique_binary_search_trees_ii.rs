/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, generate all structurally unique BST's (binary search trees) that store values 1 ... n.
 *
 * Example:
 *
 *
 * Input: 3
 * Output:
 * [
 *   [1,null,3,2],
 *   [3,2,null,1],
 *   [3,1,null,null,2],
 *   [2,1,3],
 *   [1,null,2,null,3]
 * ]
 * Explanation:
 * The above output corresponds to the 5 unique BST's shown below:
 *
 *    1         3     3      2      1
 *     \       /     /      / \      \
 *      3     2     1      1   3      2
 *     /     /       \                 \
 *    2     1         2                 3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-binary-search-trees-ii/
// discuss: https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
  1

  1       2
   \     /
    2   1

 1         3     3      2      1
  \       /     /      / \      \
   3     2     1      1   3      2
  /     /       \                 \
 2     1         2                 3

  4  1
 /    \
(5*)   3
      / \
     2   4
 */
use crate::util::tree::{to_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n < 1 {
            return vec![];
        }
        let mut res = vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))];
        for val in 2..n + 1 {
            let mut next = Vec::new();
            for root in res.into_iter() {
                let mut dummy = Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })));
                let mut parent = dummy.as_ref().unwrap().clone();
                let mut node = root;
                // we know that val is larger than all the elements in the tree
            }
            res = next;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_95() {}
}

/**
 * [99] Recover Binary Search Tree
 *
 * Two elements of a binary search tree (BST) are swapped by mistake.
 *
 * Recover the tree without changing its structure.
 *
 * Example 1:
 *
 *
 * Input: [1,3,null,null,2]
 *
 *    1
 *   /
 *  3
 *   \
 *    2
 *
 * Output: [3,1,null,null,2]
 *
 *    3
 *   /
 *  1
 *   \
 *    2
 *
 *
 * Example 2:
 *
 *
 * Input: [3,1,4,null,null,2]
 *
 *   3
 *  / \
 * 1   4
 *    /
 *   2
 *
 * Output: [2,1,4,null,null,3]
 *
 *   2
 *  / \
 * 1   4
 *    /
 *   3
 *
 *
 * Follow up:
 *
 *
 * 	A solution using O(n) space is pretty straight forward.
 * 	Could you devise a constant space solution?
 *
 *
 */
pub struct Solution {}

use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/recover-binary-search-tree/
// discuss: https://leetcode.com/problems/recover-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
基本思路: 对 root, 收集左右两侧的节点值:

 - 右侧有值小于 root, 左侧有值大于 root: 交换这两个值
 - 一侧有值不合法, 另一侧合法: 交换不合法的值与 root
 - 两侧都合法: 递归对左右子树进行上述操作

直接使用上述思路会重复迭代子树, 因此可以先用一次中序遍历, 将值的分布记录在数组中, 之后可以用 O(1) 的时间得到某个 node 两侧的值分布

这种办法的时间空间复杂度都是 O(N)

原题中的 follow up 部分提出可以想一个常数空间复杂度的算法, 因此可以继续优化.

最开始的办法会重复迭代子树是因为 Top-down, 可以尝试一下 Bottom-up 能不能解决问题:

- 后序遍历递归校验每个 node 是否合法
- 假如 node 合法, 那么只需要把这个子树中的最大值和最小值返回到上一个层级来帮助判断上一个层级的子树是否合法即可, 无需记录这个子树的所有值
- 假如 node 不合法, 按最开始的办法进行处理, 除非...
- 只有一侧值不合法, 这时候由于是 Bottom-up, 不能直接交换不合法的值与 root, 而要判断交换后能否合法(Top-down 的办法中由于题目给定了有且仅有 swap 一次, 因此与 root 交换必然是合法的)

这个办法时间复杂度 O(N), 空间 O(1). 题解就用 Bottom-up 来写.
 */
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Solution::recover_helper(root.as_ref());
    }

    fn recover_helper(
        root: Option<&Rc<RefCell<TreeNode>>>,
    ) -> (
        Option<Rc<RefCell<TreeNode>>>,
        Option<Rc<RefCell<TreeNode>>>,
        bool,
    ) {
        if let Some(node) = root {
            let (l_min, l_max, l_flag) = Solution::recover_helper(node.borrow().left.as_ref());
            let (r_min, r_max, r_flag) = Solution::recover_helper(node.borrow().right.as_ref());
            // we've already find a swap, return quickly
            if l_flag || r_flag {
                return (None, None, true);
            }
            let root_val = node.borrow().val;
            let l_err = l_max.as_ref().map_or(false, |v| v.borrow().val > root_val);
            let r_err = r_min.as_ref().map_or(false, |v| v.borrow().val < root_val);
            // invalid sub-tree found, do swap
            if l_err || r_err {
                if l_err && r_err {
                } else if l_err {
                    std::mem::swap(
                        &mut l_max.unwrap().borrow_mut().val,
                        &mut node.borrow_mut().val,
                    );
                } else if r_err {
                    std::mem::swap(
                        &mut r_min.unwrap().borrow_mut().val,
                        &mut node.borrow_mut().val,
                    )
                }
                return (None, None, true);
            }
            (
                if l_min.is_some() {
                    l_min
                } else {
                    Some(node.clone())
                },
                if r_max.is_some() {
                    r_max
                } else {
                    Some(node.clone())
                },
                false,
            )
        } else {
            (None, None, false)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_99() {
        let mut tree = tree![3, 1, 4, null, null, 2];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![2, 1, 4, null, null, 3]);

        let mut tree = tree![2, 6, 5, null, null, 3, 1, null, 4];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![2, 1, 5, null, null, 3, 6, null, 4]);
    }
}

/**
 * [148] Sort List
 *
 * Sort a linked list in O(n log n) time using constant space complexity.
 *
 * Example 1:
 *
 *
 * Input: 4->2->1->3
 * Output: 1->2->3->4
 *
 *
 * Example 2:
 *
 *
 * Input: -1->5->3->4->0
 * Output: -1->0->3->4->5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/sort-list/
// discuss: https://leetcode.com/problems/sort-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
堆排序需要额外空间, 不行

快排:
  * 不行, 单链表要快排必须同时持有两个 mut 引用, 而 rust 里这是不可能的(不知道用 unsafe 行不行)
  * 不用 rust 的话应该是可行的, Lomuto-partition, 用一个慢指针记录 no_lager_than 位置

归并，有点慢, 每次切分要遍历找到切分点, 而且递归栈深度 O(logN) 也不算严格的 O(1) 空间

Rust 标准库的 std::collections::LinkedList 都没有实现 sort() 你敢信...

这题用 rust 解对我而言真的是 Hard 级而不是 Medium 级了...

这里也是前置知识不足, GG 了解到链表的最佳排序方式确实就是 merge-sort
*/
impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            len += 1;
            ptr = node.next.as_ref();
        }
        Solution::merge_sort(head, len)
    }

    fn merge_sort(mut head: Option<Box<ListNode>>, len: i32) -> Option<Box<ListNode>> {
        if len < 2 {
            return head;
        }
        let mut next = head.as_mut();
        let mut i = 1;
        while i < len / 2 {
            next = next.unwrap().next.as_mut();
            i += 1;
        }
        let mut l2 = next.unwrap().next.take();
        let mut l1 = Solution::merge_sort(head, len / 2);
        let mut l2 = Solution::merge_sort(l2, len - len / 2);
        Solution::merge(l1, l2)
    }

    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        loop {
            match (l1, l2) {
                (Some(mut node1), Some(mut node2)) => {
                    let node = if node1.val > node2.val {
                        // give back ownership
                        l2 = node2.next.take();
                        l1 = Some(node1);
                        node2
                    } else {
                        l1 = node1.next.take();
                        l2 = Some(node2);
                        node1
                    };
                    next.as_mut().unwrap().next = Some(node);
                    next = next.unwrap().next.as_mut();
                }
                (Some(mut node1), None) => {
                    next.unwrap().next = Some(node1);
                    break;
                }
                (None, Some(mut node2)) => {
                    next.unwrap().next = Some(node2);
                    break;
                }
                (None, None) => break,
            }
        }
        dummy.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_148() {
        assert_eq!(
            Solution::sort_list(linked![4, 2, 1, 3]),
            linked![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::sort_list(linked![-1, 5, 3, 4, 0]),
            linked![-1, 0, 3, 4, 5]
        );
        assert_eq!(Solution::sort_list(linked![]), linked![]);
    }
}

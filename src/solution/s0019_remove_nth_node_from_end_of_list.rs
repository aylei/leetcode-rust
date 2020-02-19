/**
 * [19] Remove Nth Node From End of List
 *
 * Given a linked list, remove the n-th node from the end of list and return its head.
 *
 * Example:
 *
 *
 * Given linked list: 1-&gt;2-&gt;3-&gt;4-&gt;5, and n = 2.
 *
 * After removing the second node from the end, the linked list becomes 1-&gt;2-&gt;3-&gt;5.
 *
 *
 * Note:
 *
 * Given n will always be valid.
 *
 * Follow up:
 *
 * Could you do this in one pass?
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// discuss: https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// one pass (two pointer runner pattern) cannot make borrow checker happy
// but two pass don't takes longer time
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        {
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }
        let idx = len - n;
        {
            let mut p = dummy_head.as_mut();
            for _ in 0..(idx) {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}

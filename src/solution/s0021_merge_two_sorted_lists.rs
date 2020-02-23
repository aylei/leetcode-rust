/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/merge-two-sorted-lists/
// discuss: https://leetcode.com/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// recursive will be much easier to understand
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = &mut dummy_head;
        let (mut l1, mut l2) = (l1, l2);
        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                head.as_mut().unwrap().next = l2;
                break;
            } else if l2.is_none() {
                head.as_mut().unwrap().next = l1;
                break;
            }
            let next = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let (origin, next) = Solution::take_head(l1);
                l1 = origin;
                next
            } else {
                let (origin, next) = Solution::take_head(l2);
                l2 = origin;
                next
            };
            head.as_mut().unwrap().next = next;
            head = &mut head.as_mut().unwrap().next;
        }
        dummy_head.unwrap().next
    }

    #[inline(always)]
    fn take_head(mut l: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let l_next = l.as_mut().unwrap().next.take();
        let next = l.take();
        l = l_next;
        (l, next)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}

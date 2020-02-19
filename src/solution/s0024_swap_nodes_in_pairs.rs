/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 *
 * Example:
 *
 *
 * Given 1->2->3->4, you should return the list as 2->1->4->3.
 *
 * Note:
 *
 *
 * 	Your algorithm should use only constant extra space.
 * 	You may not modify the values in the list's nodes, only nodes itself may be changed.
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/swap-nodes-in-pairs/
// discuss: https://leetcode.com/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();
        loop {
            let mut left = head.as_mut().unwrap().next.take();
            if left.is_none() {
                break;
            }
            let mut right = left.as_mut().unwrap().next.take();
            // handle the un-paired one, e.g. [1, 2, 3] -> [2, 1, 3], 3 is un-paired
            if right.is_none() {
                head.as_mut().unwrap().next = left;
                break;
            }
            let mut next = right.as_mut().unwrap().next.take();
            // BEFORE: head -> left -> right -> next
            // AFTER: head -> right -> left -> next
            left.as_mut().unwrap().next = next;
            right.as_mut().unwrap().next = left;
            head.as_mut().unwrap().next = right;
            head = head.unwrap().next.as_mut().unwrap().next.as_mut();
        }
        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}

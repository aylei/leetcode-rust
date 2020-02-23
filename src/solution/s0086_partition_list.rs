/**
 * [86] Partition List
 *
 * Given a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 *
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *
 * Example:
 *
 *
 * Input: head = 1->4->3->2->5->2, x = 3
 * Output: 1->2->2->4->3->5
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/partition-list/
// discuss: https://leetcode.com/problems/partition-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lower = Some(Box::new(ListNode::new(0)));
        let mut higher = Some(Box::new(ListNode::new(0)));
        let mut lower_tail = lower.as_mut();
        let mut higher_tail = higher.as_mut();
        let mut head = head;
        while let Some(mut inner) = head {
            let mut next = inner.next.take();
            if inner.val < x {
                lower_tail.as_mut().unwrap().next = Some(inner);
                lower_tail = lower_tail.unwrap().next.as_mut();
            } else {
                higher_tail.as_mut().unwrap().next = Some(inner);
                higher_tail = higher_tail.unwrap().next.as_mut();
            }
            head = next
        }
        lower_tail.as_mut().unwrap().next = higher.unwrap().next.take();
        lower.unwrap().next.take()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_86() {
        assert_eq!(
            Solution::partition(linked![1, 4, 3, 2, 5, 2], 3),
            linked![1, 2, 2, 4, 3, 5]
        );
        assert_eq!(
            Solution::partition(linked![1, 4, 3, 2, 5, 2], 8),
            linked![1, 4, 3, 2, 5, 2]
        );
        assert_eq!(Solution::partition(linked![], 0), linked![]);
    }
}

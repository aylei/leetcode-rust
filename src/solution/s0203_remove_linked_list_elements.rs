/**
 * [203] Remove Linked List Elements
 *
 * Remove all elements from a linked list of integers that have value val.
 *
 * Example:
 *
 *
 * Input:  1->2->6->3->4->5->6, val = 6
 * Output: 1->2->3->4->5
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/remove-linked-list-elements/
// discuss: https://leetcode.com/problems/remove-linked-list-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        while let Some(mut inner) = head {
            head = inner.next.take();
            if inner.val != val {
                next.as_mut().unwrap().next = Some(inner);
                next = next.unwrap().next.as_mut();
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
    fn test_203() {
        assert_eq!(
            Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6),
            linked![1, 2, 3, 4, 5]
        );
    }
}

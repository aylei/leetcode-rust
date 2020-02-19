/**
 * [143] Reorder List
 *
 * Given a singly linked list L: L0&rarr;L1&rarr;&hellip;&rarr;Ln-1&rarr;Ln,<br />
 * reorder it to: L0&rarr;Ln&rarr;L1&rarr;Ln-1&rarr;L2&rarr;Ln-2&rarr;&hellip;
 *
 * You may not modify the values in the list's nodes, only nodes itself may be changed.
 *
 * Example 1:
 *
 *
 * Given 1->2->3->4, reorder it to 1->4->2->3.
 *
 * Example 2:
 *
 *
 * Given 1->2->3->4->5, reorder it to 1->5->2->4->3.
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reorder-list/
// discuss: https://leetcode.com/problems/reorder-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
 1->2->3->4->5

 1->2->3<-4<-5

 1->5->2->4->3
*/
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // TODO
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_143() {}
}

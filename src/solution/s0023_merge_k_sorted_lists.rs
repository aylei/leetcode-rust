/**
 * [23] Merge k Sorted Lists
 *
 * Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/merge-k-sorted-lists/
// discuss: https://leetcode.com/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// head value and the index
struct Node(i32, usize);

// sort in reverse order of head value
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0).reverse())
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        for (idx, node) in lists.iter().enumerate() {
            node.as_ref()
                .and_then(|n| Some(heap.push(Node(n.val, idx))));
        }
        Solution::next(lists, &mut heap)
    }

    fn next(
        mut lists: Vec<Option<Box<ListNode>>>,
        heap: &mut BinaryHeap<Node>,
    ) -> Option<Box<ListNode>> {
        heap.pop().map(|node| {
            let next = lists[node.1].take().unwrap().next;
            next.as_ref()
                .and_then(|n| Some(heap.push(Node(n.val, node.1))));
            lists[node.1] = next;
            Box::new(ListNode {
                val: node.0,
                next: Solution::next(lists, heap),
            })
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}

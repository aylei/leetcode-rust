/**
 * [147] Insertion Sort List
 *
 * Sort a linked list using insertion sort.
 *
 * <ol>
 * </ol>
 *
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif" style="height:180px; width:300px" /><br />
 * <small>A graphical example of insertion sort. The partial sorted list (black) initially contains only the first element in the list.<br />
 * With each iteration one element (red) is removed from the input data and inserted in-place into the sorted list</small><br />
 *
 *
 * <ol>
 * </ol>
 *
 * Algorithm of Insertion Sort:
 *
 * <ol>
 * 	Insertion sort iterates, consuming one input element each repetition, and growing a sorted output list.
 * 	At each iteration, insertion sort removes one element from the input data, finds the location it belongs within the sorted list, and inserts it there.
 * 	It repeats until no input elements remain.
 * </ol>
 *
 * <br />
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
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/insertion-sort-list/
// discuss: https://leetcode.com/problems/insertion-sort-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO; boring
impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_147() {}
}

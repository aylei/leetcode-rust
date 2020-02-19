/**
 * [215] Kth Largest Element in an Array
 *
 * Find the kth largest element in an unsorted array. Note that it is the kth largest element in the sorted order, not the kth distinct element.
 *
 * Example 1:
 *
 *
 * Input: [3,2,1,5,6,4] and k = 2
 * Output: 5
 *
 *
 * Example 2:
 *
 *
 * Input: [3,2,3,1,2,4,5,5,6] and k = 4
 * Output: 4
 *
 * Note: <br />
 * You may assume k is always valid, 1 &le; k &le; array's length.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-largest-element-in-an-array/
// discuss: https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for &num in nums.iter() {
            if heap.len() < k as usize {
                heap.push(Reverse(num))
            } else if num >= heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        heap.peek().unwrap().0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_215() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}

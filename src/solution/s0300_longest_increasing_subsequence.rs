/**
 * [300] Longest Increasing Subsequence
 *
 * Given an unsorted array of integers, find the length of longest increasing subsequence.
 *
 * Example:
 *
 *
 * Input: [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 *
 * Note:
 *
 *
 * 	There may be more than one LIS combination, it is only necessary for you to return the length.
 * 	Your algorithm should run in O(n^2) complexity.
 *
 *
 * Follow up: Could you improve it to O(n log n) time complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-subsequence/
// discuss: https://leetcode.com/problems/longest-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// N^2, DP: L[i] = max(1 + L[j]) for j in [0, i) and nums[j] < nums[i]
// N * logN, kick out strategy, maintain an increasing array, new elements kick out a formal one larger than it, if the new element is largest, expand the array
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut incr = Vec::new();
        for &num in nums.iter() {
            if let Err(idx) = incr.binary_search(&num) {
                if idx >= incr.len() {
                    incr.push(num);
                } else {
                    incr[idx] = num;
                }
            }
        }
        incr.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_300() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}

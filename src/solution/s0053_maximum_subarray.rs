/**
 * [53] Maximum Subarray
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 *
 * Example:
 *
 *
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 *
 *
 * Follow up:
 *
 * If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray/
// discuss: https://leetcode.com/problems/maximum-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut j = 0_usize;
        let mut max = i32::min_value();
        let mut curr = 0;
        for j in 0..nums.len() {
            curr += nums[j];
            max = i32::max(max, curr);
            if curr <= 0 {
                curr = 0;
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![-8]), -8);
        assert_eq!(Solution::max_sub_array(vec![-8, -2]), -2);
    }
}

/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is greater than its neighbors.
 *
 * Given an input array nums, where nums[i] &ne; nums[i+1], find a peak element and return its index.
 *
 * The array may contain multiple peaks, in that case return the index to any one of the peaks is fine.
 *
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 1 or 5
 * Explanation: Your function can return either index number 1 where the peak element is 2,
 *              or index number 5 where the peak element is 6.
 *
 *
 * Note:
 *
 * Your solution should be in logarithmic complexity.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-peak-element/
// discuss: https://leetcode.com/problems/find-peak-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0_usize, nums.len() - 1);
        let mut mid = 0;
        while lo < hi {
            mid = (hi - lo) / 2 + lo;
            if mid + 1 < nums.len() && nums[mid] < nums[mid + 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}

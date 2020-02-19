/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of n positive integers and a positive integer s, find the minimal length of a contiguous subarray of which the sum &ge; s. If there isn't one, return 0 instead.
 *
 * Example:
 *
 *
 * Input: s = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: the subarray [4,3] has the minimal length under the problem constraint.
 *
 * <div class="spoilers">Follow up:</div>
 *
 * <div class="spoilers">If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log n). </div>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0_usize, 0_usize);
        let mut min = i32::max_value();
        let mut found = false;
        let mut sum = 0;
        while j < nums.len() {
            sum += nums[j];
            if sum >= s {
                found = true;
                while i <= j {
                    sum -= nums[i];
                    i += 1;
                    if sum < s {
                        min = i32::min(min, j as i32 - i as i32 + 2);
                        break;
                    }
                }
            }
            j += 1;
        }
        if found {
            min
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }
}

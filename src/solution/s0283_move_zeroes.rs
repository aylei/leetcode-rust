/**
 * [283] Move Zeroes
 *
 * Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 *
 * Example:
 *
 *
 * Input: [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 *
 * Note:
 *
 * <ol>
 * 	You must do this in-place without making a copy of the array.
 * 	Minimize the total number of operations.
 * </ol>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/move-zeroes/
// discuss: https://leetcode.com/problems/move-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_none_zero = 0_usize;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(last_none_zero, i);
                last_none_zero += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut vec = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 3, 12, 0, 0]);
    }
}

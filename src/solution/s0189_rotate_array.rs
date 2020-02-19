/**
 * [189] Rotate Array
 *
 * Given an array, rotate the array to the right by k steps, where k is non-negative.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3,4,5,6,7] and k = 3
 * Output: [5,6,7,1,2,3,4]
 * Explanation:
 * rotate 1 steps to the right: [7,1,2,3,4,5,6]
 * rotate 2 steps to the right: [6,7,1,2,3,4,5]
 * rotate 3 steps to the right: [5,6,7,1,2,3,4]
 *
 *
 * Example 2:
 *
 *
 * Input: [-1,-100,3,99] and k = 2
 * Output: [3,99,-1,-100]
 * Explanation:
 * rotate 1 steps to the right: [99,-1,-100,3]
 * rotate 2 steps to the right: [3,99,-1,-100]
 *
 *
 * Note:
 *
 *
 * 	Try to come up as many solutions as you can, there are at least 3 different ways to solve this problem.
 * 	Could you do it in-place with O(1) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-array/
// discuss: https://leetcode.com/problems/rotate-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let offset = (k as usize) % nums.len();
        if offset == 0 {
            return;
        }
        let mut idx = 0;
        let mut num = nums[0];
        let mut start_idx = 0;
        for _ in 0..nums.len() {
            idx = (idx + offset) % nums.len();
            let temp = num;
            num = nums[idx];
            nums[idx] = temp;
            if idx == start_idx {
                idx += 1;
                start_idx = idx;
                num = nums[idx];
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_189() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![5, 6, 1, 2, 3, 4]);
    }
}

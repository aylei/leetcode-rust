/**
 * [55] Jump Game
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 *
 * Each element in the array represents your maximum jump length at that position.
 *
 * Determine if you are able to reach the last index.
 *
 * Example 1:
 *
 *
 * Input: [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 *
 * Example 2:
 *
 *
 * Input: [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum
 *              jump length is 0, which makes it impossible to reach the last index.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game/
// discuss: https://leetcode.com/problems/jump-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_idx = 0_usize;
        let mut start = 0_usize;
        while start < nums.len() && start <= max_idx {
            max_idx = usize::max(start + nums[start] as usize, max_idx);
            start += 1;
        }
        return max_idx >= (nums.len() - 1);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 0, 0, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![8, 3, 1, 1, 0, 0, 0, 4]), true);
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 0, 1, 1]), true);
        assert_eq!(
            Solution::can_jump(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]),
            true
        );
    }
}

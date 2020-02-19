/**
 * [213] House Robber II
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
 *
 * Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.
 *
 * Example 1:
 *
 *
 * Input: [2,3,2]
 * Output: 3
 * Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2),
 *              because they are adjacent houses.
 *
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 *              Total amount you can rob = 1 + 3 = 4.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber-ii/
// discuss: https://leetcode.com/problems/house-robber-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// DP twice: rob first one || not rob first one
// F[i] = max(F[i-2] + num[i], F[i-1])
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max = i32::min_value();
        for &rob_first in vec![true, false].iter() {
            let (mut prev, mut curr) = (0, 0);
            for (k, &num) in nums.iter().enumerate() {
                if k == 0 && !rob_first {
                    continue;
                }
                // k is last element but not the first element
                if k != 0 && k == (nums.len() - 1) && rob_first {
                    continue;
                }
                let next = i32::max(prev + num, curr);
                prev = curr;
                curr = next;
            }
            max = i32::max(max, curr)
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_213() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}

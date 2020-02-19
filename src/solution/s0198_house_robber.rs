/**
 * [198] House Robber
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
 *
 * Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 *              Total amount you can rob = 1 + 3 = 4.
 *
 * Example 2:
 *
 *
 * Input: [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
 *              Total amount you can rob = 2 + 9 + 1 = 12.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber/
// discuss: https://leetcode.com/problems/house-robber/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
动态规划, 记抢到第 i 户为止的最大收益为 F[i], 则:

i 有两种情况, 抢或不抢, 抢的话则最大收益是 F[i-2] + nums[i],
不抢则保持和前一次结束的收益一致, 等于 F[i-1], 于是:

F[i] = i32::max(nums[i] + F[i-2], F[i-1])

观察到 F[i] 只依赖 F[i-1] 和 F[i-2], 可以用常数空间复杂度完成
*/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut former_max = 0;
        let mut curr_max = 0;
        for &num in nums.iter() {
            let mut temp = curr_max;
            curr_max = i32::max(former_max + num, curr_max);
            former_max = temp;
        }
        curr_max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_198() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![2, 7, 9, 10, 1]), 17);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}

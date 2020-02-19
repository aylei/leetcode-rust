/**
 * [312] Burst Balloons
 *
 * Given n balloons, indexed from 0 to n-1. Each balloon is painted with a number on it represented by array nums. You are asked to burst all the balloons. If the you burst balloon i you will get nums[left] * nums[i] * nums[right] coins. Here left and right are adjacent indices of i. After the burst, the left and right then becomes adjacent.
 *
 * Find the maximum coins you can collect by bursting the balloons wisely.
 *
 * Note:
 *
 *
 * 	You may imagine nums[-1] = nums[n] = 1. They are not real therefore you can not burst them.
 * 	0 &le; n &le; 500, 0 &le; nums[i] &le; 100
 *
 *
 * Example:
 *
 *
 * Input: [3,1,5,8]
 * Output: 167
 * Explanation: nums = [3,1,5,8] --> [3,5,8] -->   [3,8]   -->  [8]  --> []
 *              coins =  3*1*5      +  3*5*8    +  1*3*8      + 1*8*1   = 167
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/burst-balloons/
// discuss: https://leetcode.com/problems/burst-balloons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
The key idea is, for a sequence of balloon, select a balloon to be the last one to be bursted:

max of [1 . a b c d e f . 1]

                ^   say we select 'c' as the last balloon to burst, then:

=
   max of [1 . a b . c] +

   max of [c . d e f . 1] +

   1 * c * 1

Then we can use memorize to record the max of every sub sequence
*/
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut coins = vec![0; nums.len() + 2];
        let mut len = 0_usize;
        // filter out zeros
        for &num in nums.iter() {
            if num != 0 {
                len += 1;
                coins[len] = num;
            }
        }
        coins[0] = 1;
        coins[len + 1] = 1;

        let mut memo = vec![vec![0; len + 1]; len + 1];
        Solution::max_subrange(&coins, 1, len, &mut memo)
    }

    fn max_subrange(coins: &Vec<i32>, start: usize, end: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[start][end] != 0 {
            return memo[start][end];
        }
        if start == end {
            memo[start][end] = coins[start - 1] * coins[start] * coins[start + 1];
            return memo[start][end];
        }
        let mut max = 0;
        for i in start..end + 1 {
            let left_max = if i > start {
                Solution::max_subrange(coins, start, i - 1, memo)
            } else {
                0
            };
            let right_max = if i < end {
                Solution::max_subrange(coins, i + 1, end, memo)
            } else {
                0
            };
            max = i32::max(
                max,
                left_max + right_max + coins[i] * coins[start - 1] * coins[end + 1],
            );
        }
        memo[start][end] = max;
        return memo[start][end];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_312() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}

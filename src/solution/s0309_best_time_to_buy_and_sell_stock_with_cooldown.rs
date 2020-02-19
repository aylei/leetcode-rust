/**
 * [309] Best Time to Buy and Sell Stock with Cooldown
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
 *
 *
 * 	You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
 * 	After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)
 *
 *
 * Example:
 *
 *
 * Input: [1,2,3,0,2]
 * Output: 3
 * Explanation: transactions = [buy, sell, cooldown, buy, sell]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
dp[i]: max profit with selling at day i
dp2[i]: max profit till day i

dp[i] = max(dp[i-1] + p[i] - p[i-1], dp2[i-2], dp2[i-3] + p[i] - p[i-1])
*/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        if prices.len() == 2 {
            return i32::max(0, prices[1] - prices[0]);
        }
        let mut dp = vec![0; prices.len()];
        let mut dp2 = vec![0; prices.len()];
        let mut max = 0;
        dp[0] = 0;
        dp2[0] = 0;
        dp[1] = prices[1] - prices[0];
        dp2[1] = i32::max(dp2[0], dp[1]);
        dp[2] = i32::max(prices[2] - prices[1], prices[2] - prices[0]);
        dp2[2] = i32::max(dp2[1], dp[2]);
        for i in 3..prices.len() {
            dp[i] = i32::max(
                dp[i - 1] + prices[i] - prices[i - 1],
                i32::max(dp2[i - 2], dp2[i - 3] + prices[i] - prices[i - 1]),
            );
            dp2[i] = i32::max(dp2[i - 1], dp[i]);
        }
        let mut temp = 0;
        for &m in dp.iter() {
            if m > temp {
                temp = m;
            }
        }
        temp
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_309() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![4, 2, 7, 1, 11]), 10);
    }
}

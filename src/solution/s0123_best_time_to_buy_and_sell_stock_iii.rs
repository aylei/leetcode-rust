/**
 * [123] Best Time to Buy and Sell Stock III
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete at most two transactions.
 *
 * Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
 *
 * Example 1:
 *
 *
 * Input: [3,3,5,0,0,3,1,4]
 * Output: 6
 * Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 *              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 *              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
 *              engaging multiple transactions at the same time. You must sell before buying again.
 *
 *
 * Example 3:
 *
 *
 * Input: [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
先考虑只进行 1 次交易的情况, 我们求以 i *为售出点*, 只进行 1 次交易获得的最大利润, 那么:

f[i] = if f[i-1] > 0 { f[i-1] } else { 0 } + prices[i] - prices[i-1]

这很容易解, 解完之后找出 f 里的最大值即可, 但这不容易推广到 K 次交易的情况, 因为这时 f[i] 不代表到 i *为止*的最大利润, 无法作为单独的交易帮助递推
(到 i 为止的含义是售出点可以在 [0,i] 之间)

我们可以稍作改进, 变成求以 i 为结束点, 只进行 1 次交易获得的最大利润, 那么:

f[i] = max(
   f[i-1],
   prices[i] - min(prices[j] { j in [0, i-1] })
)

这仍然是一个 O(N) 的解法, 因为 min(prices[j] { j in [0, i-1] }) 不需要遍历, 可以在递推过程中直接维护好

现在再推广到进行 K 次交易的情况, 那我们要求以 i 为结束点, 进行 k 次交易获得的最大利润, 这时有了变化, 我们可以在 j 之前再进行 K - 1 次交易:

f[k, i] = max(
   f[k, i-1],
   prices[i] + max(f[k-1, j] - prices[j]) { j in [0, i-1] } )
)

显然, f[0, i] = 0, f[k, 0] = 0

这个算法可以形象地描述一下, 在 k = 1 时, 我们每次要找的就是 i 之前的最低谷点作为这次交易的开始点 j, 而当 k > 1 时,
我们 i 之前就有可能已经进行过交易了, 这时我们在找开始点 j 时, 就要同时考虑 "直到 j 为止, k-1 次交易的最大收益" - "j 本身的值". 以此来找到一个最佳点 j

在实现时, 假如用 Bottom-Up 递推, 那么只需要维护一个 vec[i], 因为每轮递推时只会考虑上一轮的数据, 我们可以复用这个 O(N) 的额外存储空间
*/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let max_trans = 2;
        let mut cache = vec![0; prices.len()];
        for trans in 0..max_trans {
            // best_by_in 维护了考虑前 N 次交易的最佳的买入点, 即 max(f[k-1, j] - prices[j]) { j in [0, i-1] }
            let mut best_buy_in = cache[0] - prices[0];
            for i in 1..prices.len() {
                // 复用 vec 前暂存一下前一次的计算结果
                let temp = cache[i];
                cache[i] = i32::max(cache[i - 1], best_buy_in + prices[i]);
                // 更新 best_buy_in
                best_buy_in = i32::max(best_buy_in, temp - prices[i]);
            }
        }
        return *cache.last().unwrap();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
}

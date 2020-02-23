/**
 * [188] Best Time to Buy and Sell Stock IV
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete at most k transactions.
 *
 * Note:<br />
 * You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
 *
 * Example 1:
 *
 *
 * Input: [2,4,1], k = 2
 * Output: 2
 * Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
 *
 *
 * Example 2:
 *
 *
 * Input: [3,2,6,5,0,3], k = 2
 * Output: 7
 * Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4.
 *              Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
已经在 #123 里解过了, 为了方便阅读直接把那题的分析拷贝到这里

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

最后, 这题会给 k 非常大的 corner case, 实际上 k 大于 prices.len() / 2 后面就没有意义了, 可以 shortcut 掉(== 允许无穷次交易的场景), 下面写的比较糙,
直接限制了一下循环次数, 实际跑的时候运行时间会长一点
*/
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let max_trans = k as usize;
        let mut cache = vec![0; prices.len()];
        for _ in 0..usize::min(max_trans, prices.len() / 2 + 1) {
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
    fn test_188() {
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}

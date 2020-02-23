/**
 * [115] Distinct Subsequences
 *
 * Given a string S and a string T, count the number of distinct subsequences of S which equals T.
 *
 * A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ACE" is a subsequence of "ABCDE" while "AEC" is not).
 *
 * Example 1:
 *
 *
 * Input: S = "rabbbit", T = "rabbit"
 * Output: 3
 * Explanation:
 *
 * As shown below, there are 3 ways you can generate "rabbit" from S.
 * (The caret symbol ^ means the chosen letters)
 *
 * rabbbit
 * ^^^^ ^^
 * rabbbit
 * ^^ ^^^^
 * rabbbit
 * ^^^ ^^^
 *
 *
 * Example 2:
 *
 *
 * Input: S = "babgbag", T = "bag"
 * Output: 5
 * Explanation:
 *
 * As shown below, there are 5 ways you can generate "bag" from S.
 * (The caret symbol ^ means the chosen letters)
 *
 * babgbag
 * ^^ ^
 * babgbag
 * ^^    ^
 * babgbag
 * ^    ^^
 * babgbag
 *   ^  ^^
 * babgbag
 *     ^^^
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences/
// discuss: https://leetcode.com/problems/distinct-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
首先想到 DFS. 但这里 DFS 有重复计算, 因为我们不需要列出所有的路径, 复杂度可以考虑 "aaaaaaaaaaaaaaaaaaaa"
里找 "aaaaaaaaa", 直接搜索的话复杂度是指数级的(阶乘), 原因很明显, 这本身是个排列组合, 可以套 combination 公式
20! / ((20-10)! * 10!) 得到结果是 184756

要把复杂度从指数级降下来, 那么必须干掉重复计算, 那就想到 memorize, 想到 memorize 就想到 DP 和 Bottom-Up 递推,
回顾一下 #62 和 #63 这两个问题 (unique paths), 使用的是Bottom-Up DP, 到达每个格子的可能路径是上下两个格子的
可能路径的和. 这里就跳过了很多的计算, 不需要把每条路径都遍历出来了. 在 unique paths 问题中, 到达右下角的路径数的
子问题是到达右下角左侧格子的路径数以及到达右下角上侧格子的路径数. 这个问题也是一样的道理, s 中找子序列 t:

* s[0..i] 包含的 t 序列数就是所有 s[0..j] (j < i)  包含的 t[0..t.len()-1] 的序列数

以 babgbag 中找 bag 为例, 做一次 Bottom-Up 递推:

  b a b g b a g
b 1   1   1       3    找 'b' 这个子序列, 那么以 [0, 2, 4] 这三个下标结尾各有一种
a   1       3     4    找 'ba' 这个子序列, 那么以 1 结尾有1种(0 < 1), 以 5 结尾有 3 种 (0,2,4 都 < 5)
g       1     4   5    同理, 以 3 结尾有 1 种, 以 6 结尾有 4 种, 共 5 种

显然, 计算第 N 行时只依赖第 N-1 行的结果, 因此我们不需要存储整个矩阵, 只需要存储一行即可

时间复杂度是 O(M*N), 空间复杂度是 O(M)
*/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut cache = vec![0; s.len()];
        for (i, ch) in t.chars().into_iter().enumerate() {
            let mut acc = 0;
            // first char initialization
            if i == 0 {
                for i in 0..s.len() {
                    if ch == s[i] {
                        cache[i] = 1
                    }
                }
                continue;
            }
            for i in 0..s.len() {
                let new_acc = acc + cache[i];
                cache[i] = if s[i] == ch { acc } else { 0 };
                acc = new_acc;
            }
        }
        cache.into_iter().fold(0, |acc, x| acc + x)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_115() {
        //assert_eq!(Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned()), 3);
        assert_eq!(
            Solution::num_distinct("babgbag".to_owned(), "bag".to_owned()),
            5
        );
        assert_eq!(
            Solution::num_distinct("aaaaaaaaaaaaaaaaaaaa".to_owned(), "aaaaaaaaaa".to_owned()),
            184756
        );
    }
}

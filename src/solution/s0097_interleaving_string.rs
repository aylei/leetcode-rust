/**
 * [97] Interleaving String
 *
 * Given s1, s2, s3, find whether s3 is formed by the interleaving of s1 and s2.
 *
 * Example 1:
 *
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/interleaving-string/
// discuss: https://leetcode.com/problems/interleaving-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// DFS with memorization
/*
思路: DFS, 三个指针 i,j,k 分别指向 s1, s2, s3 已经消费到的 char 位置, 下一个可以走的路径是 s3 当前消费到的 char 值

如 aaaaaas     aaaaaaaw    aaaaaaaaaaaaaasw
那么第一步可以从 s1 或 s2 取一个 char, 用 DFS 的方式搜索整个解空间

优化: 直接 DFS 非常慢, 还是上面的例子, 最差情况是大量重复字符, 时间复杂度直接是 2^(M+N), 优化方式借鉴 DP 经常用到的
memorize, 使用一个二维数组缓存每一对遍历过的 i,j 最后是否能产生合法的 interleaving.

优化后通过缓存剪除的路径比较难分析, 但很显然能知道最差情况也只需要将所有 M*N 的组合进行标记, 因此最差时间复杂度 O(M*N)
空间复杂度 O(M*N)
*/

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut cache = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        Solution::dfs(
            &s1.chars().collect(),
            &s2.chars().collect(),
            &s3.chars().collect(),
            0,
            0,
            0,
            &mut cache,
        )
    }

    fn dfs(
        s1: &Vec<char>,
        s2: &Vec<char>,
        s3: &Vec<char>,
        i: usize,
        j: usize,
        k: usize,
        invalid: &mut Vec<Vec<bool>>,
    ) -> bool {
        if invalid[i][j] {
            return false;
        }
        if i == s1.len() && j == s2.len() && k == s3.len() {
            return true;
        }
        let valid = (i < s1.len()
            && k < s3.len()
            && s1[i] == s3[k]
            && Solution::dfs(s1, s2, s3, i + 1, j, k + 1, invalid))
            || (j < s2.len()
                && k < s3.len()
                && s2[j] == s3[k]
                && Solution::dfs(s1, s2, s3, i, j + 1, k + 1, invalid));
        if !valid {
            invalid[i][j] = true
        }
        valid
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_97() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
        assert_eq!(
            Solution::is_interleave("a".to_owned(), "b".to_owned(), "a".to_owned()),
            false
        );
    }
}

/**
 * [132] Palindrome Partitioning II
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 *
 * Return the minimum cuts needed for a palindrome partitioning of s.
 *
 * Example:
 *
 *
 * Input: "aab"
 * Output: 1
 * Explanation: The palindrome partitioning ["aa","b"] could be produced using 1 cut.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning-ii/
// discuss: https://leetcode.com/problems/palindrome-partitioning-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
为了方便讨论, 我们记 n 个字符的最少回文分段是 f(n), 则切分次数为 f(n)-1, 接下来递推 f(n):

f(n) = min(f(n-i) + 1) { i in [0..n] and s[i..n] is palindrome }

显然, f(1) 为 1, f(0) 为 0

判断 is_palindrome 也需要优化, 使用一个备忘录, 将判断回文的操作优化到 O(1):

is_palindrome(s[i..n]) = s[i] == s[n] && is_palindrome(s[i+1..n-1])

最后的复杂度: 时间 O(N^2), 空间 O(N^2)
*/
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        if s.is_empty() {
            return 0;
        }
        let mut palindrome_cache: Vec<Vec<Option<bool>>> = vec![vec![None; s.len()]; s.len()];
        let mut min = Vec::with_capacity(s.len() + 1);
        min.push(0);
        min.push(1);
        for i in 1..s.len() {
            let mut local_min = i32::max_value();
            for j in 0..i + 1 {
                if Solution::is_palindrome(&mut palindrome_cache, &s, j, i) {
                    local_min = i32::min(1 + min[j], local_min);
                }
            }
            min.push(local_min);
        }
        min[s.len()] - 1
    }

    fn is_palindrome(
        cache: &mut Vec<Vec<Option<bool>>>,
        s: &Vec<char>,
        i: usize,
        j: usize,
    ) -> bool {
        if j <= i {
            return true;
        }
        if let Some(result) = cache[i][j] {
            result
        } else {
            let result = s[i] == s[j]
                && (i + 1 > s.len() || j < 1 || Solution::is_palindrome(cache, s, i + 1, j - 1));
            cache[i][j] = Some(result);
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_132() {
        assert_eq!(Solution::min_cut("aab".to_owned()), 1);
        assert_eq!(Solution::min_cut("aaa".to_owned()), 0);
        assert_eq!(Solution::min_cut("aabb".to_owned()), 1);
    }
}

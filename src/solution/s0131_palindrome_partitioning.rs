/**
 * [131] Palindrome Partitioning
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 *
 * Return all possible palindrome partitioning of s.
 *
 * Example:
 *
 *
 * Input: "aab"
 * Output:
 * [
 *   ["aa","b"],
 *   ["a","a","b"]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning/
// discuss: https://leetcode.com/problems/palindrome-partitioning/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
记 n 个字符的回文拆分方式是 f(n) 种, 则:

f(n) = (0..n).iter().fold(0, |acc, i| {
   if is_palindrome(s[i..n]) { acc + f(i-1) } else { acc }
})

按这种方式向上递推即可, 时间复杂度为 O(N^3), 空间复杂度 O(N), 显然, is_palindrome 这一步仍然有重复计算

is_palindrome(s[i..n]) = s[i] == s[n] && is_palindrome(s[i+1..n-1])

存储所有 i, n 的 is_palindrome 结果, 则可以优化 is_palindrome 的时间到 O(1)

最后的复杂度: 时间 O(N^2), 空间 O(N^2)
*/
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().collect::<Vec<_>>();
        if s.is_empty() {
            return Vec::new();
        }
        let mut palindrome_cache = vec![vec![None; s.len()]; s.len()];
        let mut res: Vec<Vec<Vec<(usize, usize)>>> = Vec::with_capacity(s.len());
        res.push(vec![vec![(0, 1)]]);
        for n in 1..s.len() {
            let mut curr: Vec<Vec<(usize, usize)>> = Vec::new();
            for i in 0..n + 1 {
                if Solution::is_palindrome(&mut palindrome_cache, &s, i, n) {
                    if i > 0 {
                        for vec in res[i - 1].iter() {
                            let mut new_vec = vec.clone();
                            new_vec.push((i, n + 1));
                            curr.push(new_vec);
                        }
                    } else {
                        curr.push(vec![(i, n + 1)]);
                    }
                }
            }
            res.push(curr);
        }
        (*res[s.len() - 1])
            .into_iter()
            .map(|vec| {
                vec.iter()
                    .map(|&range| s[range.0..range.1].iter().collect::<String>())
                    .collect::<Vec<_>>()
            })
            .collect()
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
    fn test_131() {
        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![vec_string!["aa", "b"], vec_string!["a", "a", "b"],]
        );
        assert_eq!(
            Solution::partition("aaa".to_owned()),
            vec![
                vec_string!["aaa"],
                vec_string!["a", "aa"],
                vec_string!["aa", "a"],
                vec_string!["a", "a", "a"],
            ]
        );
    }
}

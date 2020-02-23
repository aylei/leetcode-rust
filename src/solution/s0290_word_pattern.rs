/**
 * [290] Word Pattern
 *
 * Given a pattern and a string str, find if str follows the same pattern.
 *
 * Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in str.
 *
 * Example 1:
 *
 *
 * Input: pattern = "abba", str = "dog cat cat dog"
 * Output: true
 *
 * Example 2:
 *
 *
 * Input:pattern = "abba", str = "dog cat cat fish"
 * Output: false
 *
 * Example 3:
 *
 *
 * Input: pattern = "aaaa", str = "dog cat cat dog"
 * Output: false
 *
 * Example 4:
 *
 *
 * Input: pattern = "abba", str = "dog dog dog dog"
 * Output: false
 *
 * Notes:<br />
 * You may assume pattern contains only lowercase letters, and str contains lowercase letters that may be separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-pattern/
// discuss: https://leetcode.com/problems/word-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut p2s = HashMap::new();
        let mut s2p = HashMap::new();
        let mut s_iter = s.split(" ");
        let mut p_iter = pattern.chars();
        loop {
            match (s_iter.next(), p_iter.next()) {
                (Some(sub), Some(ch)) => {
                    if *p2s.entry(ch).or_insert(sub) != sub || *s2p.entry(sub).or_insert(ch) != ch {
                        return false;
                    }
                }
                (None, None) => break,
                _ => return false,
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_290() {
        assert_eq!(
            Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()),
            true
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()),
            false
        );
        assert_eq!(
            Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()),
            false
        );
    }
}

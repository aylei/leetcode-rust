/**
 * [205] Isomorphic Strings
 *
 * Given two strings s and t, determine if they are isomorphic.
 *
 * Two strings are isomorphic if the characters in s can be replaced to get t.
 *
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character but a character may map to itself.
 *
 * Example 1:
 *
 *
 * Input: s = "egg", t = "add"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "foo", t = "bar"
 * Output: false
 *
 * Example 3:
 *
 *
 * Input: s = "paper", t = "title"
 * Output: true
 *
 * Note:<br />
 * You may assume both s and t have the same length.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/isomorphic-strings/
// discuss: https://leetcode.com/problems/isomorphic-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::char;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        Solution::code(s) == Solution::code(t)
    }

    fn code(s: String) -> String {
        let mut map = HashMap::new();
        let mut start: char = '0';
        let mut res = String::new();
        for ch in s.chars() {
            let v = map.entry(ch).or_insert_with(|| {
                start = ((start as u8) + 1) as char;
                start
            });
            res.push(*v)
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_owned(), "app".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("pecil".to_owned(), "this".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
    }
}

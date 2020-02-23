/**
 * [76] Minimum Window Substring
 *
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 *
 * Example:
 *
 *
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 *
 *
 * Note:
 *
 *
 * 	If there is no such window in S that covers all characters in T, return the empty string "".
 * 	If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-window-substring/
// discuss: https://leetcode.com/problems/minimum-window-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() || t.len() > s.len() {
            return "".to_owned();
        }
        let (mut start, mut end) = (0_usize, 0_usize);
        let mut result = (0_usize, 0_usize);
        loop {}
        s[result.0..result.1].to_owned()
    }

    fn count_char(s: String) -> HashMap<char, i32> {
        let mut res = HashMap::new();
        for ch in s.chars().into_iter() {
            *res.entry(ch).or_insert(0) += 1;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {}
}

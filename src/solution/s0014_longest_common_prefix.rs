/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 * Example 1:
 *
 *
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 * Note:
 *
 * All given inputs are in lowercase letters a-z.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-prefix/
// discuss: https://leetcode.com/problems/longest-common-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::str::Chars;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        let mut iters: Vec<Chars> = strs.iter().map(|s| s.chars()).collect();
        let mut curr_char: Option<char> = None;
        if strs.len() < 1 {
            return prefix;
        }
        loop {
            curr_char.take().map(|ch| prefix.push(ch));
            for iter in iters.iter_mut() {
                let mut ch = iter.next();
                if ch.is_none() {
                    return prefix;
                }
                match curr_char {
                    None => curr_char = ch.take(),
                    Some(curr) => {
                        if curr != ch.unwrap() {
                            return prefix;
                        }
                    }
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}

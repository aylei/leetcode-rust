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
        let mut chars = strs.first().expect("Ok").clone();
        strs.iter().map(|v| v.chars()).fold(chars, |prev, cur: Chars| {
            let len = longest_len(&(prev.chars()), &cur);
            let mut ch = String::from("");
            if len == 0 {
                return ch;
            }
            


            let mut iter_prev = prev.chars().into_iter();
            let mut iter_cur = cur.into_iter();

            for i in 0..=len-1 {
                let p = iter_prev.next().unwrap();
                if p == iter_cur.next().unwrap() {
                   ch.push(p);
                } else {
                    break;
                }
            }
            return ch;
        })
    }
}
pub fn longest_len<'a>(a: &'a Chars, b: &'a Chars) -> usize {
    let a_len = a.clone().count();
    let b_len = b.clone().count();
    if a_len > b_len {
        return b_len;
    }
    a_len
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

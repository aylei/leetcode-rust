/**
 * [28] Implement strStr()
 *
 * Implement <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strStr()</a>.
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 *
 * Example 1:
 *
 *
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 *
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strstr()</a> and Java's <a href="https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)" target="_blank">indexOf()</a>.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-strstr/
// discuss: https://leetcode.com/problems/implement-strstr/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        haystack.find(&needle).map_or(-1_i32, |v| v as i32)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2)
    }
}

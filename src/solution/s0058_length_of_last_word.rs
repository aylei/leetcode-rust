/**
 * [58] Length of Last Word
 *
 * Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.
 *
 * If the last word does not exist, return 0.
 *
 * Note: A word is defined as a character sequence consists of non-space characters only.
 *
 * Example:
 *
 * Input: "Hello World"
 * Output: 5
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-last-word/
// discuss: https://leetcode.com/problems/length-of-last-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let seq: Vec<char> = s.chars().rev().collect();
        let mut result = 0;
        let mut find = false;
        for ch in seq {
            if ch == ' ' && find {
                break;
            }
            if ch != ' ' {
                find = true;
                result += 1;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("       ".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("     rrrrr  ".to_owned()), 5);
    }
}

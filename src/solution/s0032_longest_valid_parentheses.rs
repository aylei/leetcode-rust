/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *
 * Example 1:
 *
 *
 * Input: "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()"
 *
 *
 * Example 2:
 *
 *
 * Input: ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()"
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-valid-parentheses/
// discuss: https://leetcode.com/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// time: O(N) space: O(1)
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut seq: Vec<char> = s.chars().collect();
        let forward_max = Solution::longest(&seq, '(');
        seq.reverse();
        let backward_max = Solution::longest(&seq, ')');
        i32::max(forward_max, backward_max)
    }

    fn longest(seq: &Vec<char>, plus_char: char) -> i32 {
        let mut stack = 0;
        let mut max_len = 0;
        let (mut i, mut j) = (0_usize, 0_usize);
        while j < seq.len() {
            if seq[j] == plus_char {
                stack += 1;
            } else {
                // stack exhausted, shift over
                if stack < 1 {
                    i = j + 1;
                } else {
                    stack -= 1;
                    if stack == 0 {
                        max_len = i32::max(max_len, (j - i + 1) as i32);
                    }
                }
            }
            j += 1;
        }
        max_len
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses("(((((()()".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_valid_parentheses("((((((((()))".to_string()),
            6
        );
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(")()(((())))(".to_string()),
            10
        );
        assert_eq!(
            Solution::longest_valid_parentheses("(()(((()".to_string()),
            2
        );
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}

/**
 * [227] Basic Calculator II
 *
 * Implement a basic calculator to evaluate a simple expression string.
 *
 * The expression string contains only non-negative integers, +, -, *, / operators and empty spaces  . The integer division should truncate toward zero.
 *
 * Example 1:
 *
 *
 * Input: "3+2*2"
 * Output: 7
 *
 *
 * Example 2:
 *
 *
 * Input: " 3/2 "
 * Output: 1
 *
 * Example 3:
 *
 *
 * Input: " 3+5 / 2 "
 * Output: 5
 *
 *
 * Note:
 *
 *
 * 	You may assume that the given expression is always valid.
 * 	Do not use the eval built-in library function.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator-ii/
// discuss: https://leetcode.com/problems/basic-calculator-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut acc = 0_i64;
        let mut prev = 0_i64;
        let mut curr = 0_i64;
        let mut sign = 1;
        let mut has_prev = false;
        let mut multiple = true;
        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    curr = 10 * curr + (ch as u8 - '0' as u8) as i64;
                }
                '+' | '-' => {
                    if has_prev {
                        if multiple {
                            curr = prev * curr;
                        } else {
                            curr = prev / curr;
                        }
                        has_prev = false;
                    }
                    acc += curr * sign;
                    curr = 0;
                    sign = if ch == '+' { 1 } else { -1 };
                }
                '*' | '/' => {
                    if has_prev {
                        if multiple {
                            curr = prev * curr;
                        } else {
                            curr = prev / curr;
                        }
                    }
                    has_prev = true;
                    prev = curr;
                    curr = 0;
                    multiple = if ch == '*' { true } else { false };
                }
                _ => {}
            }
        }
        if has_prev {
            if multiple {
                curr = prev * curr;
            } else {
                curr = prev / curr;
            }
        }
        acc += sign * curr;
        acc as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_227() {
        assert_eq!(Solution::calculate("3+2*2".to_owned()), 7);
    }
}

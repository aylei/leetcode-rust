/**
 * [8] String to Integer (atoi)
 *
 * Implement <code><span>atoi</span></code> which converts a string to an integer.
 *
 * The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
 *
 * The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
 *
 * If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
 *
 * If no valid conversion could be performed, a zero value is returned.
 *
 * Note:
 *
 * <ul>
 * 	<li>Only the space character <code>&#39; &#39;</code> is considered as whitespace character.</li>
 * 	<li>Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. If the numerical value is out of the range of representable values, INT_MAX (2^31 - 1) or INT_MIN (-2^31) is returned.</li>
 * </ul>
 *
 * Example 1:
 *
 *
 * Input: "42"
 * Output: 42
 *
 *
 * Example 2:
 *
 *
 * Input: "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is &#39;-&#39;, which is the minus sign.
 *              Then take as many numerical digits as possible, which gets 42.
 *
 *
 * Example 3:
 *
 *
 * Input: "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit &#39;3&#39; as the next character is not a numerical digit.
 *
 *
 * Example 4:
 *
 *
 * Input: "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is &#39;w&#39;, which is not a numerical
 *              digit or a +/- sign. Therefore no valid conversion could be performed.
 *
 * Example 5:
 *
 *
 * Input: "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
 *              Thefore INT_MIN (-2^31) is returned.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(input: String) -> i32 {
        let (i32_min, i32_max) = (-2_i64.pow(31), 2_i64.pow(31) - 1);
        let mut result: i64 = 0;
        let mut minus = false;
        // simple state machine
        let mut num_matched = false;
        for ch in input.chars().into_iter() {
            if !num_matched {
                match ch {
                    ' ' => {}
                    '0'..='9' => {
                        num_matched = true;
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                    }
                    '-' => {
                        num_matched = true;
                        minus = true;
                    }
                    '+' => {
                        num_matched = true;
                    }
                    _ => return 0,
                }
            } else {
                match ch {
                    '0'..='9' => {
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                        if result > i32_max {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
        result = if minus { -result } else { result };
        if result > i32_max {
            return i32_max as i32;
        }
        if result < i32_min {
            return i32_min as i32;
        }
        return result as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
    }
}

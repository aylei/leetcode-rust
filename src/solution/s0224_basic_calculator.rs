/**
 * [224] Basic Calculator
 *
 * Implement a basic calculator to evaluate a simple expression string.
 *
 * The expression string may contain open ( and closing parentheses ), the plus + or minus sign -, non-negative integers and empty spaces  .
 *
 * Example 1:
 *
 *
 * Input: "1 + 1"
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: " 2-1 + 2 "
 * Output: 3
 *
 * Example 3:
 *
 *
 * Input: "(1+(4+5+2)-3)+(6+8)"
 * Output: 23
 * Note:
 *
 *
 * 	You may assume that the given expression is always valid.
 * 	Do not use the eval built-in library function.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator/
// discuss: https://leetcode.com/problems/basic-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(PartialEq, Copy, Clone, Debug)]
enum Token {
    LeftBracket,
    RightBracket,
    PlusSign,
    MinusSign,
    Number(i64),
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        // lexer
        let mut token_stream = Vec::new();
        let mut num = 0_i64;
        let mut in_num = false;
        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    in_num = true;
                    num = 10 * num + (ch as u8 - '0' as u8) as i64;
                }
                _ => {
                    if in_num {
                        token_stream.push(Token::Number(num));
                        num = 0;
                        in_num = false;
                    }
                    match ch {
                        '(' => {
                            token_stream.push(Token::LeftBracket);
                        }
                        ')' => {
                            token_stream.push(Token::RightBracket);
                        }
                        '+' => {
                            token_stream.push(Token::PlusSign);
                        }
                        '-' => {
                            token_stream.push(Token::MinusSign);
                        }
                        _ => {}
                    };
                }
            }
        }
        if in_num {
            token_stream.push(Token::Number(num));
        }

        // parser
        let mut stack = Vec::new();
        let mut iter = token_stream.into_iter();
        let mut pause = false;
        let mut token = Token::LeftBracket;
        loop {
            if !pause {
                token = if let Some(token) = iter.next() {
                    token
                } else {
                    break;
                }
            } else {
                pause = false;
            }
            match token {
                Token::LeftBracket => {
                    stack.push(token);
                }
                Token::RightBracket => {
                    if let Token::Number(right_hand) = stack.pop().unwrap() {
                        stack.pop();
                        pause = true;
                        token = Token::Number(right_hand);
                    }
                }
                Token::PlusSign => {
                    stack.push(token);
                }
                Token::MinusSign => {
                    stack.push(token);
                }
                Token::Number(num) => {
                    if stack.is_empty() || Token::LeftBracket == *stack.last().unwrap() {
                        stack.push(Token::Number(num));
                    } else {
                        let sign = stack.pop().unwrap();
                        if let Token::Number(left_hand) = stack.pop().unwrap() {
                            let res =
                                left_hand + num * if Token::PlusSign == sign { 1 } else { -1 };
                            stack.push(Token::Number(res));
                        }
                    }
                }
            }
        }
        if let Token::Number(num) = stack.pop().unwrap() {
            return num as i32;
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("1+1".to_owned()), 2);
        assert_eq!(Solution::calculate("0".to_owned()), 0);
        assert_eq!(Solution::calculate("2147483647".to_owned()), 2147483647);
    }
}

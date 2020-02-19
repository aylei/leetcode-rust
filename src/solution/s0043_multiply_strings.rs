/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 *
 * Example 1:
 *
 *
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 *
 * Example 2:
 *
 *
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *
 *
 * Note:
 *
 * <ol>
 * 	The length of both num1 and num2 is < 110.
 * 	Both num1 and num2 contain only digits 0-9.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 * 	You must not use any built-in BigInteger library or convert the inputs to integer directly.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO
use std::char::from_digit;
use std::collections::VecDeque;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut num1: Vec<u32> = num1.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
        let mut num2: Vec<u32> = num2.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
        let mut buffer = VecDeque::with_capacity(num2.len() + 1);
        let mut res: Vec<char> = Vec::new();
        let mut carry = 0_u32;
        num1.reverse();
        num2.reverse();
        for (i, multiplier) in num1.into_iter().enumerate() {
            buffer
                .pop_back()
                .and_then(|digit| Some(res.push(from_digit(digit, 10).unwrap())));
            for &multiplicand in num2.iter() {}
        }
        res.reverse();
        res.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {}
}

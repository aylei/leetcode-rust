/**
 * [67] Add Binary
 *
 * Given two binary strings, return their sum (also a binary string).
 *
 * The input strings are both non-empty and contains only characters 1 or 0.
 *
 * Example 1:
 *
 *
 * Input: a = "11", b = "1"
 * Output: "100"
 *
 * Example 2:
 *
 *
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-binary/
// discuss: https://leetcode.com/problems/add-binary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::char::from_digit;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut buf = Vec::with_capacity(usize::max(a.len(), b.len()) + 1);
        let mut a: Vec<char> = a.chars().collect();
        let mut b: Vec<char> = b.chars().collect();
        let mut carry = 0;
        while !(a.is_empty() && b.is_empty()) {
            let mut sum = a.pop().map_or(0, |ch| ch.to_digit(10).unwrap())
                + b.pop().map_or(0, |ch| ch.to_digit(10).unwrap())
                + carry;
            if sum > 1 {
                sum -= 2;
                carry = 1;
            } else {
                carry = 0;
            }
            buf.push(from_digit(sum, 10).unwrap())
        }
        if carry > 0 {
            buf.push('1')
        }
        buf.into_iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(
            Solution::add_binary("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101".to_owned()
        );
        assert_eq!(
            Solution::add_binary("11".to_owned(), "1".to_owned()),
            "100".to_owned()
        );
        assert_eq!(
            Solution::add_binary("1111".to_owned(), "1111".to_owned()),
            "11110".to_owned()
        );
    }
}

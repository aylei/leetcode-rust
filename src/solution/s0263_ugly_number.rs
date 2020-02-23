/**
 * [263] Ugly Number
 *
 * Write a program to check whether a given number is an ugly number.
 *
 * Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.
 *
 * Example 1:
 *
 *
 * Input: 6
 * Output: true
 * Explanation: 6 = 2 &times; 3
 *
 * Example 2:
 *
 *
 * Input: 8
 * Output: true
 * Explanation: 8 = 2 &times; 2 &times; 2
 *
 *
 * Example 3:
 *
 *
 * Input: 14
 * Output: false
 * Explanation: 14 is not ugly since it includes another prime factor 7.
 *
 *
 * Note:
 *
 * <ol>
 * 	1 is typically treated as an ugly number.
 * 	Input is within the 32-bit signed integer range: [-2^31,  2^31 - 1].
 * </ol>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ugly-number/
// discuss: https://leetcode.com/problems/ugly-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num <= 0 {
            false
        } else if num == 1 {
            true
        } else if num % 5 == 0 {
            Solution::is_ugly(num / 5)
        } else if num % 3 == 0 {
            Solution::is_ugly(num / 3)
        } else if num % 2 == 0 {
            Solution::is_ugly(num / 2)
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_263() {
        assert_eq!(Solution::is_ugly(25), true);
    }
}

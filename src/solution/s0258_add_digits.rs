/**
 * [258] Add Digits
 *
 * Given a non-negative integer num, repeatedly add all its digits until the result has only one digit.
 *
 * Example:
 *
 *
 * Input: 38
 * Output: 2
 * Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2.
 *              Since 2 has only one digit, return it.
 *
 *
 * Follow up:<br />
 * Could you do it without any loop/recursion in O(1) runtime?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-digits/
// discuss: https://leetcode.com/problems/add-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        1 + ((num - 1) % 9)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_258() {
        assert_eq!(Solution::add_digits(1234), 1);
    }
}

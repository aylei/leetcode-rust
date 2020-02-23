/**
 * [172] Factorial Trailing Zeroes
 *
 * Given an integer n, return the number of trailing zeroes in n!.
 *
 * Example 1:
 *
 *
 * Input: 3
 * Output: 0
 * Explanation: 3! = 6, no trailing zero.
 *
 * Example 2:
 *
 *
 * Input: 5
 * Output: 1
 * Explanation: 5! = 120, one trailing zero.
 *
 * Note: Your solution should be in logarithmic time complexity.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/factorial-trailing-zeroes/
// discuss: https://leetcode.com/problems/factorial-trailing-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut five_count = 0;
        let mut n = n;
        while n > 0 {
            n = n / 5;
            five_count += n;
        }
        five_count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_172() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(20), 4);
        assert_eq!(Solution::trailing_zeroes(1808548329), 452137076);
    }
}

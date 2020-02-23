/**
 * [231] Power of Two
 *
 * Given an integer, write a function to determine if it is a power of two.
 *
 * Example 1:
 *
 *
 * Input: 1
 * Output: true
 * Explanation: 2^0 = 1
 *
 *
 * Example 2:
 *
 *
 * Input: 16
 * Output: true
 * Explanation: 2^4 = 16
 *
 * Example 3:
 *
 *
 * Input: 218
 * Output: false
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-two/
// discuss: https://leetcode.com/problems/power-of-two/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let results: Vec<i32> = (0..31).map(|x| 2_i32.pow(x)).collect();
        results.binary_search(&n).is_ok()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_231() {
        assert_eq!(Solution::is_power_of_two(-1), false);
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(1024), true);
    }
}

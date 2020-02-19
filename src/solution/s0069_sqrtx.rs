/**
 * [69] Sqrt(x)
 *
 * Implement int sqrt(int x).
 *
 * Compute and return the square root of x, where x is guaranteed to be a non-negative integer.
 *
 * Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.
 *
 * Example 1:
 *
 *
 * Input: 4
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since
 *              the decimal part is truncated, 2 is returned.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sqrtx/
// discuss: https://leetcode.com/problems/sqrtx/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Newton-Raphson for: root^2 - n = 0
// Tangent equation: y = 2 * root * x - (root^2 + n)
// Zero point: (root^2 + n) / (2 * root)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut size = x;
        let mut base = 1;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if mid <= x / mid {
                base = mid;
            }
            size -= half;
        }
        base
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(16), 4);
        assert_eq!(Solution::my_sqrt(17), 4);
        assert_eq!(Solution::my_sqrt(81), 9);
        assert_eq!(Solution::my_sqrt(82), 9);
        assert_eq!(Solution::my_sqrt(100480577), 10024);
        assert_eq!(Solution::my_sqrt(100480575), 10023);
        assert_eq!(Solution::my_sqrt(100480575), 10023);
        assert_eq!(Solution::my_sqrt(80), 8);
        assert_eq!(Solution::my_sqrt(2), 1);
    }
}

/**
 * [201] Bitwise AND of Numbers Range
 *
 * Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.
 *
 * Example 1:
 *
 *
 * Input: [5,7]
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: [0,1]
 * Output: 0
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bitwise-and-of-numbers-range/
// discuss: https://leetcode.com/problems/bitwise-and-of-numbers-range/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// just find the highest bit 1 of m and n
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut m = m;
        let mut n = n;
        if m == 0 {
            return 0;
        }
        let mut step = 1;
        while m != n {
            // shortcut
            if m == 0 {
                return 0;
            }
            m >>= 1;
            n >>= 1;
            step <<= 1;
        }
        return m * step;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_201() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }
}

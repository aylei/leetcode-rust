/**
 * [264] Ugly Number II
 *
 * Write a program to find the n-th ugly number.
 *
 * Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.
 *
 * Example:
 *
 *
 * Input: n = 10
 * Output: 12
 * Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.
 *
 * Note:
 *
 * <ol>
 * 	1 is typically treated as an ugly number.
 * 	n does not exceed 1690.
 * </ol>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ugly-number-ii/
// discuss: https://leetcode.com/problems/ugly-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut base2 = 0;
        let mut base3 = 0;
        let mut base5 = 0;
        let mut ugly = vec![1; 1];
        for _ in 1..n {
            let next2 = ugly[base2] * 2;
            let next3 = ugly[base3] * 3;
            let next5 = ugly[base5] * 5;
            let next = i32::min(next2, i32::min(next3, next5));
            if next2 == next {
                base2 += 1
            }
            if next3 == next {
                base3 += 1
            }
            if next5 == next {
                base5 += 1
            }
            ugly.push(next)
        }
        *ugly.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}

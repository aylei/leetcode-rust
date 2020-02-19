/**
 * [202] Happy Number
 *
 * Write an algorithm to determine if a number is "happy".
 *
 * A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.
 *
 * Example:
 *
 *
 * Input: 19
 * Output: true
 * Explanation:
 * 1^2 + 9^2 = 82
 * 8^2 + 2^2 = 68
 * 6^2 + 8^2 = 100
 * 1^2 + 0^2 + 0^2 = 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/happy-number/
// discuss: https://leetcode.com/problems/happy-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut n = n;
        loop {
            set.insert(n);
            let temp = Solution::next(n);
            if temp == 1 {
                return true;
            }
            if !set.insert(temp) {
                return false;
            }
            n = temp
        }
        return false;
    }

    fn next(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;
        while n != 0 {
            res += (n % 10).pow(2);
            n = n / 10;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(235123), false);
    }
}

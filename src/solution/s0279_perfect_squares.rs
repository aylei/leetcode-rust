/**
 * [279] Perfect Squares
 *
 * Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.
 *
 * Example 1:
 *
 *
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 *
 * Example 2:
 *
 *
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-squares/
// discuss: https://leetcode.com/problems/perfect-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// BFS
use std::collections::VecDeque;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        let mut deq = VecDeque::new();
        deq.push_back((1, n));
        while let Some((level, num)) = deq.pop_front() {
            let mut base = 1;
            while base * base <= num {
                if base * base == num {
                    return level;
                }
                deq.push_back((level + 1, num - base * base));
                base += 1;
            }
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_279() {
        assert_eq!(Solution::num_squares(13), 2);
        assert_eq!(Solution::num_squares(12), 3);
    }
}

/**
 * [70] Climbing Stairs
 *
 * You are climbing a stair case. It takes n steps to reach to the top.
 *
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 *
 * Note: Given n will be a positive integer.
 *
 * Example 1:
 *
 *
 * Input: 2
 * Output: 2
 * Explanation: There are two ways to climb to the top.
 * 1. 1 step + 1 step
 * 2. 2 steps
 *
 *
 * Example 2:
 *
 *
 * Input: 3
 * Output: 3
 * Explanation: There are three ways to climb to the top.
 * 1. 1 step + 1 step + 1 step
 * 2. 1 step + 2 steps
 * 3. 2 steps + 1 step
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/climbing-stairs/
// discuss: https://leetcode.com/problems/climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Bottom-up DP
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let (mut prev, mut curr) = (1, 2);
        for i in 2..n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}

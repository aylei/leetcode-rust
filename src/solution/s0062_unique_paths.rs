/**
 * [62] Unique Paths
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 *
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 *
 * How many possible unique paths are there?
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" /><br />
 * <small>Above is a 7 x 3 grid. How many possible unique paths are there?</small>
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 *
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down
 * 2. Right -> Down -> Right
 * 3. Down -> Right -> Right
 *
 *
 * Example 2:
 *
 *
 * Input: m = 7, n = 3
 * Output: 28
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths/
// discuss: https://leetcode.com/problems/unique-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// its high school math: C(r,n) = n! / r!(n-r)! ...are you fxxking kidding me?
// ...high school math will attempt to i32 overflow, we have to do it clever
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = ((m - 1) as u64, (n - 1) as u64);
        let sum = m + n;
        (Solution::partial_factorial(u64::max(m, n), sum)
            / Solution::partial_factorial(0, u64::min(m, n))) as i32
    }

    #[inline(always)]
    pub fn partial_factorial(start: u64, mut end: u64) -> u64 {
        if start > end {
            unreachable!()
        }
        let mut res = 1;
        while end > start {
            println!("{}", end);
            res *= end;
            end -= 1;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(2, 2), 2);
        assert_eq!(Solution::unique_paths(36, 7), 4496388);
    }
}

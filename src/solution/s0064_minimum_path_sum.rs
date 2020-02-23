/**
 * [64] Minimum Path Sum
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   [1,3,1],
 *   [1,5,1],
 *   [4,2,1]
 * ]
 * Output: 7
 * Explanation: Because the path 1&rarr;3&rarr;1&rarr;1&rarr;1 minimizes the sum.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-path-sum/
// discuss: https://leetcode.com/problems/minimum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (height, width) = (grid.len(), grid[0].len());
        let mut grid = grid;
        let mut step = 1;
        while step <= height + width - 2 {
            for x in 0..(step + 1) {
                let y = step - x;
                if x >= height || y >= width {
                    continue;
                }
                if x < 1 {
                    grid[x][y] += grid[x][y - 1];
                } else if y < 1 {
                    grid[x][y] += grid[x - 1][y];
                } else {
                    grid[x][y] += i32::min(grid[x][y - 1], grid[x - 1][y]);
                }
            }
            step += 1;
        }
        grid[height - 1][width - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64() {
        assert_eq!(Solution::min_path_sum(vec![vec![2]]), 2);
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1],]),
            7
        );
        assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1],]), 5);
    }
}

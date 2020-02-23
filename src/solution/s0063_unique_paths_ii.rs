/**
 * [63] Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 *
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 *
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" />
 *
 * An obstacle and empty space is marked as 1 and 0 respectively in the grid.
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *   [0,0,0],
 *   [0,1,0],
 *   [0,0,0]
 * ]
 * Output: 2
 * Explanation:
 * There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths-ii/
// discuss: https://leetcode.com/problems/unique-paths-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Bottom-Up DP
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (height, width) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut paths: Vec<Vec<i32>> = vec![vec![0; width]; height];
        if obstacle_grid[0][0] != 1 {
            paths[0][0] = 1;
        }
        let mut step = 1;
        while step <= height + width - 2 {
            for x in 0..(step + 1) {
                let y = step - x;
                if x >= height || y >= width || obstacle_grid[x][y] == 1 {
                    continue;
                }
                if y >= 1 {
                    paths[x][y] = paths[x][y] + paths[x][y - 1];
                }
                if x >= 1 {
                    paths[x][y] = paths[x][y] + paths[x - 1][y];
                }
            }
            step += 1;
        }
        paths[height - 1][width - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_63() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 0],]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![1, 0],]),
            0
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]),
            10
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 1, 0],
            ]),
            0
        );
    }
}

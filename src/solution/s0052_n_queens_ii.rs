/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n&times;n chessboard such that no two queens attack each other.
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 *
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *
 * Example:
 *
 *
 * Input: 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
 * [
 *  [".Q..",  // Solution 1
 *   "...Q",
 *   "Q...",
 *   "..Q."],
 *
 *  ["..Q.",  // Solution 2
 *   "Q...",
 *   "...Q",
 *   ".Q.."]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens-ii/
// discuss: https://leetcode.com/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut num = 0;
        Solution::schedule_queens(&mut board, &mut num, n as usize, 0);
        num
    }

    fn schedule_queens(board: &mut Vec<Vec<char>>, num: &mut i32, len: usize, row: usize) {
        for col in 0..len {
            if !Solution::collision(&board, len, row, col) {
                board[row][col] = 'Q';
                if row == len - 1 {
                    *num += 1;
                } else {
                    Solution::schedule_queens(board, num, len, row + 1);
                }
                board[row][col] = '.';
            }
        }
    }

    #[inline(always)]
    fn collision(board: &Vec<Vec<char>>, len: usize, x: usize, y: usize) -> bool {
        for i in 0..x {
            if board[i][y] == 'Q' {
                return true;
            }
        }
        let (mut i, mut j) = (x as i32 - 1, y as i32 - 1);
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return true;
            }
            i -= 1;
            j -= 1;
        }
        let (mut i, mut j) = (x as i32 - 1, y as i32 + 1);
        while i >= 0 && j < len as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return true;
            }
            i -= 1;
            j += 1;
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(8), 92);
        assert_eq!(Solution::total_n_queens(13), 73712);
        // assert_eq!(Solution::total_n_queens(14), 365596);
    }
}

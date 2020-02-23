/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n*n chessboard such that no two queens attack each other.
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 *
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 *
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.
 *
 * Example:
 *
 *
 * Input: 4
 * Output: [
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
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens/
// discuss: https://leetcode.com/problems/n-queens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut solution = Vec::new();
        Solution::schedule_queens(&mut board, &mut solution, n as usize, 0);
        solution
    }

    fn schedule_queens(
        board: &mut Vec<Vec<char>>,
        solution: &mut Vec<Vec<String>>,
        len: usize,
        row: usize,
    ) {
        for col in 0..len {
            if !Solution::collision(&board, len, row, col) {
                board[row][col] = 'Q';
                if row == len - 1 {
                    solution.push(board.iter().map(|vec| vec.iter().collect()).collect());
                } else {
                    Solution::schedule_queens(board, solution, len, row + 1);
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
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}

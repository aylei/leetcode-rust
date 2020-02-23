/**
 * [36] Valid Sudoku
 *
 * Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
 *
 * <ol>
 * 	Each row must contain the digits 1-9 without repetition.
 * 	Each column must contain the digits 1-9 without repetition.
 * 	Each of the 9 3x3 sub-boxes of the grid must contain the digits 1-9 without repetition.
 * </ol>
 *
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" /><br />
 * <small>A partially filled sudoku which is valid.</small>
 *
 * The Sudoku board could be partially filled, where empty cells are filled with the character '.'.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *   ["5","3",".",".","7",".",".",".","."],
 *   ["6",".",".","1","9","5",".",".","."],
 *   [".","9","8",".",".",".",".","6","."],
 *   ["8",".",".",".","6",".",".",".","3"],
 *   ["4",".",".","8",".","3",".",".","1"],
 *   ["7",".",".",".","2",".",".",".","6"],
 *   [".","6",".",".",".",".","2","8","."],
 *   [".",".",".","4","1","9",".",".","5"],
 *   [".",".",".",".","8",".",".","7","9"]
 * ]
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input:
 * [
 *   ["8","3",".",".","7",".",".",".","."],
 *   ["6",".",".","1","9","5",".",".","."],
 *   [".","9","8",".",".",".",".","6","."],
 *   ["8",".",".",".","6",".",".",".","3"],
 *   ["4",".",".","8",".","3",".",".","1"],
 *   ["7",".",".",".","2",".",".",".","6"],
 *   [".","6",".",".",".",".","2","8","."],
 *   [".",".",".","4","1","9",".",".","5"],
 *   [".",".",".",".","8",".",".","7","9"]
 * ]
 * Output: false
 * Explanation: Same as Example 1, except with the 5 in the top left corner being
 *     modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
 *
 *
 * Note:
 *
 *
 * 	A Sudoku board (partially filled) could be valid but is not necessarily solvable.
 * 	Only the filled cells need to be validated according to the mentioned rules.
 * 	The given board contain only digits 1-9 and the character '.'.
 * 	The given board size is always 9x9.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-sudoku/
// discuss: https://leetcode.com/problems/valid-sudoku/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// just brute force
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut table = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for row in board.iter() {
            for z in 1..10 {
                table[z] = 0;
            }
            for ch in row {
                match ch.to_digit(10) {
                    None => continue,
                    Some(idx) => {
                        if table[idx as usize] > 0 {
                            return false;
                        } else {
                            table[idx as usize] = 1
                        }
                    }
                }
            }
        }
        for i in 0..9 {
            for z in 1..10 {
                table[z] = 0;
            }
            for row in board.iter() {
                match row[i].to_digit(10) {
                    None => continue,
                    Some(idx) => {
                        if table[idx as usize] > 0 {
                            return false;
                        } else {
                            table[idx as usize] = 1
                        }
                    }
                }
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                for z in 1..10 {
                    table[z] = 0;
                }
                for row in 3 * i..3 * (i + 1) {
                    for column in 3 * j..3 * (j + 1) {
                        match board[row][column].to_digit(10) {
                            None => continue,
                            Some(idx) => {
                                if table[idx as usize] > 0 {
                                    return false;
                                } else {
                                    table[idx as usize] = 1
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_36() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}

/**
 * [289] Game of Life
 *
 * According to the <a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life" target="_blank">Wikipedia's article</a>: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."
 *
 * Given a board with m by n cells, each cell has an initial state live (1) or dead (0). Each cell interacts with its <a href="https://en.wikipedia.org/wiki/Moore_neighborhood" target="_blank">eight neighbors</a> (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):
 *
 * <ol>
 * 	Any live cell with fewer than two live neighbors dies, as if caused by under-population.
 * 	Any live cell with two or three live neighbors lives on to the next generation.
 * 	Any live cell with more than three live neighbors dies, as if by over-population..
 * 	Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
 * </ol>
 *
 * Write a function to compute the next state (after one update) of the board given its current state. <span>The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously.</span>
 *
 * Example:
 *
 *
 * Input:
 * <span id="example-input-1-1">[
 *   [0,1,0],
 *   [0,0,1],
 *   [1,1,1],
 *   [0,0,0]
 * ]</span>
 * Output:
 * <span id="example-output-1">[
 *   [0,0,0],
 *   [1,0,1],
 *   [0,1,1],
 *   [0,1,0]
 * ]</span>
 *
 *
 * Follow up:
 *
 * <ol>
 * 	Could you solve it in-place? Remember that the board needs to be updated at the same time: You cannot update some cells first and then use their updated values to update other cells.
 * 	In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches the border of the array. How would you address these problems?
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/game-of-life/
// discuss: https://leetcode.com/problems/game-of-life/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// in-place: 1: live->live, 0: die->die, 2: die->live, 3: live->die
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (height, width) = (board.len(), board[0].len());
        let neighbors = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for i in 0..height {
            for j in 0..width {
                let mut live = 0;
                for offset in neighbors.iter() {
                    if (offset.0 < 0 && i == 0)
                        || (offset.0 > 0 && i == height - 1)
                        || (offset.1 < 0 && j == 0)
                        || (offset.1 > 0 && j == width - 1)
                    {
                        continue;
                    }
                    let v = board[(i as i32 + offset.0) as usize][(j as i32 + offset.1) as usize];
                    if v == 1 || v == 3 {
                        live += 1;
                    }
                }
                if board[i][j] == 1 && (live < 2 || live > 3) {
                    // go die
                    board[i][j] = 3;
                } else if board[i][j] == 0 && live == 3 {
                    // go live
                    board[i][j] = 2;
                }
            }
        }

        for i in 0..height {
            for j in 0..width {
                if board[i][j] == 2 {
                    board[i][j] = 1;
                } else if board[i][j] == 3 {
                    board[i][j] = 0;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_289() {
        let mut test = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut test);
        assert_eq!(
            test,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0],]
        );
    }
}

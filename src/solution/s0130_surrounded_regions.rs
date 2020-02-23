/**
 * [130] Surrounded Regions
 *
 * Given a 2D board containing 'X' and 'O' (the letter O), capture all regions surrounded by 'X'.
 *
 * A region is captured by flipping all 'O's into 'X's in that surrounded region.
 *
 * Example:
 *
 *
 * X X X X
 * X O O X
 * X X O X
 * X O X X
 *
 *
 * After running your function, the board should be:
 *
 *
 * X X X X
 * X X X X
 * X X X X
 * X O X X
 *
 *
 * Explanation:
 *
 * Surrounded regions shouldn&rsquo;t be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/surrounded-regions/
// discuss: https://leetcode.com/problems/surrounded-regions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
从最外层开始, 基于为 'O' 的格子做 DFS, 将与边界连接的所有 'O' 标记为一个特殊 char, 最后将没有标记到的 'O' 全部标记为 'X'
*/
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let (height, width) = (board.len(), board[0].len());
        // 遍历最外层的 4 条边
        for j in 0..width {
            Solution::dfs(0, j, height, width, board);
            Solution::dfs(height - 1, j, height, width, board);
        }
        for i in 1..height - 1 {
            Solution::dfs(i, 0, height, width, board);
            Solution::dfs(i, width - 1, height, width, board);
        }
        for k in 0..height * width {
            board[k / width][k % width] = if board[k / width][k % width] == '_' {
                'O'
            } else {
                'X'
            }
        }
    }

    fn dfs(i: usize, j: usize, height: usize, width: usize, board: &mut Vec<Vec<char>>) {
        if board[i][j] == 'O' {
            board[i][j] = '_';
            if i > 1 {
                Solution::dfs(i - 1, j, height, width, board)
            }
            if j > 1 {
                Solution::dfs(i, j - 1, height, width, board)
            }
            if i + 1 < height {
                Solution::dfs(i + 1, j, height, width, board)
            }
            if j + 1 < width {
                Solution::dfs(i, j + 1, height, width, board)
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_130() {
        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X', 'O', 'X'],
            vec!['O', 'X', 'X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X', 'O', 'X'],
                vec!['O', 'X', 'X', 'O', 'O', 'X'],
                vec!['X', 'O', 'X', 'O', 'O', 'O'],
                vec!['X', 'O', 'O', 'O', 'X', 'O'],
                vec!['O', 'O', 'X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ]
            ]
        );
    }
}

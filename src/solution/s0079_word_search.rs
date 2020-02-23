/**
 * [79] Word Search
 *
 * Given a 2D board and a word, find if the word exists in the grid.
 *
 * The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *
 * Example:
 *
 *
 * board =
 * [
 *   ['A','B','C','E'],
 *   ['S','F','C','S'],
 *   ['A','D','E','E']
 * ]
 *
 * Given word = "ABCCED", return true.
 * Given word = "SEE", return true.
 * Given word = "ABCB", return false.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-search/
// discuss: https://leetcode.com/problems/word-search/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: use HashSet to record visited pos
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || word.len() < 1 {
            return false;
        }
        let (height, width) = (board.len(), board[0].len());
        if height < 1 || width < 1 {
            return false;
        }
        let seq: Vec<char> = word.chars().collect();

        for i in 0..height * width {
            if Solution::dfs(
                i / width,
                i % width,
                &seq[..],
                &board,
                vec![],
                height,
                width,
            ) {
                return true;
            }
        }
        false
    }

    fn dfs(
        x: usize,
        y: usize,
        seq: &[char],
        board: &Vec<Vec<char>>,
        mut visited: Vec<(usize, usize)>,
        height: usize,
        width: usize,
    ) -> bool {
        if seq[0] != board[x][y] {
            return false;
        }
        if seq.len() < 2 {
            return true;
        }
        visited.push((x, y));
        return (x > 0
            && !visited.contains(&(x - 1, y))
            && Solution::dfs(x - 1, y, &seq[1..], board, visited.clone(), height, width))
            || (x + 1 < height
                && !visited.contains(&(x + 1, y))
                && Solution::dfs(x + 1, y, &seq[1..], board, visited.clone(), height, width))
            || (y > 0
                && !visited.contains(&(x, y - 1))
                && Solution::dfs(x, y - 1, &seq[1..], board, visited.clone(), height, width))
            || (y + 1 < width
                && !visited.contains(&(x, y + 1))
                && Solution::dfs(x, y + 1, &seq[1..], board, visited.clone(), height, width));
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {
        assert_eq!(Solution::exist(vec![vec!['a']], "a".to_owned()), true);
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "SEE".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB".to_owned()
            ),
            false
        );
    }
}

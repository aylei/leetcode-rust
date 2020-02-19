/**
 * [200] Number of Islands
 *
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
 *
 * Example 1:
 *
 *
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * Output: 3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-islands/
// discuss: https://leetcode.com/problems/number-of-islands/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Union-Find Set
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let (height, width) = (grid.len(), grid[0].len());
        let mut parent = vec![vec![(width, height); width]; height];
        for i in 0..height {
            for j in 0..width {
                if grid[i][j] != '1' {
                    continue;
                }
                parent[i][j] = (i, j);
                if i > 0 && grid[i - 1][j] == '1' {
                    Solution::union(&mut parent, (i, j), (i - 1, j));
                }
                if j > 0 && grid[i][j - 1] == '1' {
                    Solution::union(&mut parent, (i, j), (i, j - 1));
                }
            }
        }
        let mut cnt = 0;
        for i in 0..height {
            for j in 0..width {
                if parent[i][j] == (i, j) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn get_parent(parent: &mut Vec<Vec<(usize, usize)>>, p: (usize, usize)) -> (usize, usize) {
        let mut child = p;
        let mut p = p;
        while parent[p.0][p.1] != p {
            p = parent[p.0][p.1];
        }
        // path compression, adjust all the node's parent to root along the path
        while child != p {
            let temp = parent[child.0][child.1];
            parent[child.0][child.1] = p;
            child = temp;
        }
        p
    }

    fn union(parent: &mut Vec<Vec<(usize, usize)>>, p1: (usize, usize), p2: (usize, usize)) {
        let p1 = Solution::get_parent(parent, p1);
        let p2 = Solution::get_parent(parent, p2);
        if p1 == p2 {
            return;
        }
        parent[p1.0][p1.1] = p2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_200() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '0', '0',],
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', 'o', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '1', '1',],
            ]),
            3
        );
    }
}

/**
 * [221] Maximal Square
 *
 * Given a 2D binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
 *
 * Example:
 *
 *
 * Input:
 *
 * 1 0 1 0 0
 * 1 0 <font color="red">1</font> <font color="red">1</font> 1
 * 1 1 <font color="red">1</font> <font color="red">1</font> 1
 * 1 0 0 1 0
 *
 * Output: 4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-square/
// discuss: https://leetcode.com/problems/maximal-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
DP, f(i, j) to represent the max square of matrix that end with (i, j) (right bottom corener), then:

f(0, 0) = matrix[0][0]
f(i, j) = if matrix[0][0] { min(f(i-1,j), f(i,j-1), f(i-1)(j-1)) + 1 } else { 0 }

The equation explained:

matrix:    dp:
1 1 1      1 1 1
1 1 1   -> 1 2 2
1 1 1      1 2 3
*/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (height, width) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; width]; height];
        let mut max = 0;
        for i in 0..height {
            for j in 0..width {
                if matrix[i][j] == '0' {
                    continue;
                }
                dp[i][j] = i32::min(
                    i32::min(
                        if i < 1 { 0 } else { dp[i - 1][j] },
                        if j < 1 { 0 } else { dp[i][j - 1] },
                    ),
                    if i < 1 || j < 1 { 0 } else { dp[i - 1][j - 1] },
                ) + 1;
                max = i32::max(max, dp[i][j])
            }
        }
        max * max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ]),
            4
        )
    }
}

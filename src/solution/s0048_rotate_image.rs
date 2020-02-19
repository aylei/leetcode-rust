/**
 * [48] Rotate Image
 *
 * You are given an n x n 2D matrix representing an image.
 *
 * Rotate the image by 90 degrees (clockwise).
 *
 * Note:
 *
 * You have to rotate the image <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
 *
 * Example 1:
 *
 *
 * Given input matrix =
 * [
 *   [1,2,3],
 *   [4,5,6],
 *   [7,8,9]
 * ],
 *
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [7,4,1],
 *   [8,5,2],
 *   [9,6,3]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Given input matrix =
 * [
 *   [ 5, 1, 9,11],
 *   [ 2, 4, 8,10],
 *   [13, 3, 6, 7],
 *   [15,14,12,16]
 * ],
 *
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [15,13, 2, 5],
 *   [14, 3, 4, 1],
 *   [12, 6, 8, 9],
 *   [16, 7,10,11]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-image/
// discuss: https://leetcode.com/problems/rotate-image/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// x,y ->  y,n-x     2-dimension vector rotate -90 degree:
//   ^                  x     0   1      y
//   |      |              *         =
//          v           y     -1  0      -x
// n-y,x <- n-x,n-y  if we consider axis transform, then: rotate(x, y) = (y, -x + n)

//  we only need to iterate a 1/4 corner matrix, for odd matrix, we take an extra part in x direction
//
//  even:
//
//  x x o o
//  x x o o
//  o o o o
//  o o o o
//
//  odd:
//
//  x x o o o
//  x x o o o
//  x x o o o
//  o o o o o
//  o o o o o

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut matrix = matrix;
        let (len, n) = (matrix.len(), matrix.len() - 1);
        for x in 0..len / 2 {
            for y in 0..(len + 1) / 2 {
                let temp = matrix[x][y];
                matrix[x][y] = matrix[n - y][x];
                matrix[n - y][x] = matrix[n - x][n - y];
                matrix[n - x][n - y] = matrix[y][n - x];
                matrix[y][n - x] = temp;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_48() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}

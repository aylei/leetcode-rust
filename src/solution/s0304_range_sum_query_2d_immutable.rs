/**
 * [304] Range Sum Query 2D - Immutable
 *
 * Given a 2D matrix matrix, find the sum of the elements inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
 *
 *
 * <img src="/static/images/courses/range_sum_query_2d.png" border="0" alt="Range Sum Query 2D" /><br />
 * <small>The above rectangle (with the red border) is defined by (row1, col1) = (2, 1) and (row2, col2) = (4, 3), which contains sum = 8.</small>
 *
 *
 * Example:<br>
 *
 * Given matrix = [
 *   [3, 0, 1, 4, 2],
 *   [5, 6, 3, 2, 1],
 *   [1, 2, 0, 1, 5],
 *   [4, 1, 0, 1, 7],
 *   [1, 0, 3, 0, 5]
 * ]
 *
 * sumRegion(2, 1, 4, 3) -> 8
 * sumRegion(1, 1, 2, 2) -> 11
 * sumRegion(1, 2, 2, 4) -> 12
 *
 *
 *
 * Note:<br>
 * <ol>
 * You may assume that the matrix does not change.
 * There are many calls to sumRegion function.
 * You may assume that row1 &le; row2 and col1 &le; col2.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-2d-immutable/
// discuss: https://leetcode.com/problems/range-sum-query-2d-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumMatrix {
    cache: Vec<Vec<i32>>,
}

/**               region[2, 2, 3, 4] =
 *  x x \ \ \ .     square[3,4] - square[1,4] - square[3,1] + square[1,1]
 *  x x \ \ \ .
 *  / / o o o .
 *  / / o o o .
 *  . . . . . .
 *  . . . . . .
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            return NumMatrix {
                cache: vec![vec![]],
            };
        }
        let (x_max, y_max) = (matrix.len(), matrix[0].len());
        let mut cache = vec![vec![0; y_max]; x_max];
        for x in 0..x_max {
            for y in 0..y_max {
                cache[x][y] = matrix[x][y]
                    + if y > 0 { cache[x][y - 1] } else { 0 }
                    + if x > 0 { cache[x - 1][y] } else { 0 }
                    - if x > 0 && y > 0 {
                        cache[x - 1][y - 1]
                    } else {
                        0
                    }
            }
        }
        NumMatrix { cache: cache }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        self.cache[row2][col2]
            - if row1 > 0 {
                self.cache[row1 - 1][col2]
            } else {
                0
            }
            - if col1 > 0 {
                self.cache[row2][col1 - 1]
            } else {
                0
            }
            + if row1 > 0 && col1 > 0 {
                self.cache[row1 - 1][col1 - 1]
            } else {
                0
            }
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_304() {
        let matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(matrix.sum_region(1, 2, 2, 4), 12);
    }
}

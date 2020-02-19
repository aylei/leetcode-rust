/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate a square matrix filled with elements from 1 to n^2 in spiral order.
 *
 * Example:
 *
 *
 * Input: 3
 * Output:
 * [
 *  [ 1, 2, 3 ],
 *  [ 8, 9, 4 ],
 *  [ 7, 6, 5 ]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-ii/
// discuss: https://leetcode.com/problems/spiral-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];
        if n < 1 {
            return res;
        }
        let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, n as usize, 0, n as usize);
        let mut i = 1;
        loop {
            for y in y_min..y_max {
                res[x_min][y] = i;
                i += 1;
            }
            x_min += 1;
            if x_min == x_max {
                break;
            }
            for x in x_min..x_max {
                res[x][y_max - 1] = i;
                i += 1;
            }
            y_max -= 1;
            if y_min == y_max {
                break;
            }
            for y in (y_min..y_max).rev() {
                res[x_max - 1][y] = i;
                i += 1;
            }
            x_max -= 1;
            if x_min == x_max {
                break;
            }
            for x in (x_min..x_max).rev() {
                res[x][y_min] = i;
                i += 1;
            }
            y_min += 1;
            if y_min == y_max {
                break;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5],]
        );
    }
}

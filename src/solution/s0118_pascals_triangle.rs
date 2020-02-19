/**
 * [118] Pascal's Triangle
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.
 *
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" /><br />
 * <small>In Pascal's triangle, each number is the sum of the two numbers directly above it.</small>
 *
 * Example:
 *
 *
 * Input: 5
 * Output:
 * [
 *      [1],
 *     [1,1],
 *    [1,2,1],
 *   [1,3,3,1],
 *  [1,4,6,4,1]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle/
// discuss: https://leetcode.com/problems/pascals-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if num_rows < 1 {
            return res;
        }
        let mut curr = vec![1];
        for _ in 0..num_rows {
            let mut next = vec![1; curr.len() + 1];
            for i in 1..curr.len() {
                next[i] = curr[i - 1] + curr[i];
            }
            res.push(curr);
            curr = next;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_118() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}

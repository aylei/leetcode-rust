/**
 * [119] Pascal's Triangle II
 *
 * Given a non-negative index k where k &le; 33, return the k^th index row of the Pascal's triangle.
 *
 * Note that the row index starts from 0.
 *
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" /><br />
 * <small>In Pascal's triangle, each number is the sum of the two numbers directly above it.</small>
 *
 * Example:
 *
 *
 * Input: 3
 * Output: [1,3,3,1]
 *
 *
 * Follow up:
 *
 * Could you optimize your algorithm to use only O(k) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle-ii/
// discuss: https://leetcode.com/problems/pascals-triangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
in-place algorithm

1 1 1 1 1
1 2 1 1 1
1 3 3 1 1
1 4 6 4 1
*/
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut curr = vec![1; (row_index + 1) as usize];
        for i in 0..row_index + 1 {
            let mut prev = 1;
            for j in 1..i {
                let temp = curr[j as usize];
                curr[j as usize] = temp + prev;
                prev = temp;
            }
        }
        curr
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_119() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1])
    }
}

/**
 * [120] Triangle
 *
 * Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.
 *
 * For example, given the following triangle
 *
 *
 * [
 *      [2],
 *     [3,4],
 *    [6,5,7],
 *   [4,1,8,3]
 * ]
 *
 *
 * The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11).
 *
 * Note:
 *
 * Bonus point if you are able to do this using only O(n) extra space, where n is the total number of rows in the triangle.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/triangle/
// discuss: https://leetcode.com/problems/triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![0; triangle.len()];
        for row in triangle.iter() {
            let mut prev = 0;
            for i in 0..row.len() {
                let temp = cache[i];
                cache[i] = if i == 0 {
                    cache[i]
                } else if i == row.len() - 1 {
                    prev
                } else {
                    i32::min(cache[i], prev)
                } + row[i];
                prev = temp;
            }
        }
        cache
            .into_iter()
            .fold(i32::max_value(), |min, x| i32::min(min, x))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        )
    }
}

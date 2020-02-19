/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 *
 * Example:
 *
 *
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combinations/
// discuss: https://leetcode.com/problems/combinations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        Solution::backtrack(1, n, k, vec![], &mut res);
        res
    }

    fn backtrack(start: i32, end: i32, k: i32, curr: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k < 1 {
            result.push(curr);
            return;
        }
        if end - start + 1 < k {
            // elements is not enough, return quickly
            return;
        }
        for i in start..end + 1 {
            let mut vec = curr.clone();
            vec.push(i);
            Solution::backtrack(i + 1, end, k - 1, vec, result);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_77() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(0, 1), empty);
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }
}

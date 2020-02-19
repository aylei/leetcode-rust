/**
 * [78] Subsets
 *
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: nums = [1,2,3]
 * Output:
 * [
 *   [3],
 *   [1],
 *   [2],
 *   [1,2,3],
 *   [1,3],
 *   [2,3],
 *   [1,2],
 *   []
 * ]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets/
// discuss: https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Solution::backtrack(0, vec![], &nums, &mut res);
        res
    }

    fn backtrack(start: usize, mut curr: Vec<i32>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start >= nums.len() {
            result.push(curr);
            return;
        }
        // current element dropped
        Solution::backtrack(start + 1, curr.clone(), nums, result);
        // current element picked
        curr.push(nums[start]);
        Solution::backtrack(start + 1, curr, nums, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );
    }
}

/**
 * [216] Combination Sum III
 *
 * <div>
 * Find all possible combinations of k numbers that add up to a number n, given that only numbers from 1 to 9 can be used and each combination should be a unique set of numbers.
 *
 * Note:
 *
 *
 * 	All numbers will be positive integers.
 * 	The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *
 *
 * Input: k = 3, n = 7
 * Output: [[1,2,4]]
 *
 *
 * Example 2:
 *
 *
 * Input: k = 3, n = 9
 * Output: [[1,2,6], [1,3,5], [2,3,4]]
 *
 * </div>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-iii/
// discuss: https://leetcode.com/problems/combination-sum-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if k > 9 || k < 1 {
            return vec![];
        }
        let max = (0..k).fold(0, |acc, t| acc + 9 - t);
        let min = (0..k).fold(0, |acc, t| acc + t);
        if n < min || n > max {
            return vec![];
        }
        let mut res = Vec::new();
        let mut seed = Vec::new();
        Solution::helper(n, 0, k, seed, &mut res);
        res
    }

    fn helper(distance: i32, prev: i32, remain: i32, mut curr: Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if remain == 0 {
            if distance == 0 {
                res.push(curr);
            }
            return;
        }
        for i in (prev + 1..=9) {
            if distance - i < 0 {
                break;
            }
            let mut new_vec = curr.clone();
            new_vec.push(i);
            Solution::helper(distance - i, i, remain - 1, new_vec, res);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_216() {
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }
}

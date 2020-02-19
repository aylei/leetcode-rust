/**
 * [268] Missing Number
 *
 * Given an array containing n distinct numbers taken from 0, 1, 2, ..., n, find the one that is missing from the array.
 *
 * Example 1:
 *
 *
 * Input: [3,0,1]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: [9,6,4,2,3,5,7,0,1]
 * Output: 8
 *
 *
 * Note:<br />
 * Your algorithm should run in linear runtime complexity. Could you implement it using only constant extra space complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/missing-number/
// discuss: https://leetcode.com/problems/missing-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        ((nums.len() + 1) * nums.len()) as i32 / 2 - nums.into_iter().fold(0, |acc, v| acc + v)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_268() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }
}

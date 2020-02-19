/**
 * [238] Product of Array Except Self
 *
 * Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].
 *
 * Example:
 *
 *
 * Input:  [1,2,3,4]
 * Output: [24,12,8,6]
 *
 *
 * Note: Please solve it without division and in O(n).
 *
 * Follow up:<br />
 * Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/product-of-array-except-self/
// discuss: https://leetcode.com/problems/product-of-array-except-self/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
x 2 3 4 = 24
1 x 3 4 = 12
1 2 x 4 = 8
1 2 3 x = 6
 */
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }
        let mut res = vec![1; nums.len()];
        let mut n = 1;
        for i in (0..nums.len() - 1).rev() {
            n *= nums[i + 1];
            res[i] = n;
        }
        n = 1;
        for i in 1..nums.len() {
            n *= nums[i - 1];
            res[i] *= n;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_238() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}

/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.
 *
 * Example 1:
 *
 *
 * Input: [2,3,-2,4]
 * Output: 6
 * Explanation: [2,3] has the largest product 6.
 *
 *
 * Example 2:
 *
 *
 * Input: [-2,0,-1]
 * Output: 0
 * Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-subarray/
// discuss: https://leetcode.com/problems/maximum-product-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
f[i], g[i] means the max positive value and max negative value for the sub-seq end with index i

then we have:

f[i], g[i] = if nums[i] == 0 {
   0, 0
} else if nums[i] > 0 {
       f[i-1] * nums[i], g[i-1] * nums[i]
} else if nums[i] < 0 {
       g[i-1] * nums[i], f[i-1] * nums[i]
}
*/

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut neg_max = 0;
        let mut pos_max = 0;
        for num in nums.into_iter() {
            if num == 0 {
                neg_max = 0;
                pos_max = 0;
                max = i32::max(max, 0);
            } else if num > 0 {
                pos_max = i32::max(pos_max * num, num);
                neg_max = neg_max * num;
            } else {
                let pos_pre = pos_max;
                pos_max = neg_max * num;
                neg_max = i32::min(pos_pre * num, num);
            }
            if pos_max != 0 {
                max = i32::max(max, pos_max);
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_152() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-4, -3, -2]), 12);
    }
}

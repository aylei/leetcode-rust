/**
 * [164] Maximum Gap
 *
 * Given an unsorted array, find the maximum difference between the successive elements in its sorted form.
 *
 * Return 0 if the array contains less than 2 elements.
 *
 * Example 1:
 *
 *
 * Input: [3,6,9,1]
 * Output: 3
 * Explanation: The sorted form of the array is [1,3,6,9], either
 *              (3,6) or (6,9) has the maximum difference 3.
 *
 * Example 2:
 *
 *
 * Input: [10]
 * Output: 0
 * Explanation: The array contains less than 2 elements, therefore return 0.
 *
 * Note:
 *
 *
 * 	You may assume all elements in the array are non-negative integers and fit in the 32-bit signed integer range.
 * 	Try to solve it in linear time/space.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-gap/
// discuss: https://leetcode.com/problems/maximum-gap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
/*
想不出来, 一看解析居然是 Radix Sort 或 Bucket Sort, 我就 ??? 了...

最佳算法是 Bucket Sort 吗? (桶大小取 max - min / len 那种), 看时间复杂度好像是这样

但假如整体排布非常稠密, 那么这个聪明的算法也就退化成了桶大小为 1 的桶排序
*/
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut gap = 0;
        for i in 1..nums.len() {
            gap = i32::max(nums[i] - nums[i - 1], gap);
        }
        gap
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_164() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    }
}

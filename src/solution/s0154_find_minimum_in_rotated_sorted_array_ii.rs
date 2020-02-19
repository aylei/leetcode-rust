/**
 * [154] Find Minimum in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).
 *
 * Find the minimum element.
 *
 * The array may contain duplicates.
 *
 * Example 1:
 *
 *
 * Input: [1,3,5]
 * Output: 1
 *
 * Example 2:
 *
 *
 * Input: [2,2,2,0,1]
 * Output: 0
 *
 * Note:
 *
 *
 * 	This is a follow up problem to <a href="https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/">Find Minimum in Rotated Sorted Array</a>.
 * 	Would allow duplicates affect the run-time complexity? How and why?
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
// discuss: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
针对无重复的做法, 只要二分搜索找折点即可: 假如 nums[mid] > nums[base] 那么转折点一定在右侧, 否则在左侧

但假如有重复, 就可能有 nums[mid] == nums[base], 这时就尴尬了, 无法确定转折点在左半部分还是右半部分

可以考虑一个数组, [1,1,1,1,1,1,1,0,1,1,1,1,1,1] 这个数组无论怎么去找 0, 时间复杂度无法低于 O(N)

但假如不是这种极端情况, 那么二分搜索还是能优化的, 在 153 的基础上, 碰到相等就跳过即可
*/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let mut mid = 0;
        while lo < hi {
            mid = lo + (hi - lo) / 2;
            if (nums[mid] > nums[hi]) {
                lo = mid + 1;
            } else if (nums[mid] < nums[hi]) {
                hi = mid;
            } else {
                hi -= 1;
            }
        }
        return nums[lo];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_154() {
        assert_eq!(Solution::find_min(vec![1, 2, 2, 2, 2, 2]), 1);
        assert_eq!(Solution::find_min(vec![1, 3, 3]), 1);
        assert_eq!(Solution::find_min(vec![3, 1, 3, 3]), 1);
    }
}

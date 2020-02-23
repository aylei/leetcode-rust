/**
 * [287] Find the Duplicate Number
 *
 * Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive), prove that at least one duplicate number must exist. Assume that there is only one duplicate number, find the duplicate one.
 *
 * Example 1:
 *
 *
 * Input: [1,3,4,2,2]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: [3,1,3,4,2]
 * Output: 3
 *
 * Note:
 *
 * <ol>
 * 	You must not modify the array (assume the array is read only).
 * 	You must use only constant, O(1) extra space.
 * 	Your runtime complexity should be less than O(n^2).
 * 	There is only one duplicate number in the array, but it could be repeated more than once.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-duplicate-number/
// discuss: https://leetcode.com/problems/find-the-duplicate-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// 假如把值看做 next node 的下标, 那么:
//  从 0 出发不会回到 0
//  一定有环, 因为 1-n 全部落在下标范围 [0, n] 中
//  从 0 遍历经过的环中, 一定存在重复数字 x, 且 x 就是入环点的下标:
//   1.从 0 走到入环点, 入环点的前驱值为 x; 2.入环点在环上的前驱值也是 x
//   由于我们不可能回到 0, 因此这两个节点下标不同, x 即为要找的重复数字
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow: usize = nums[0] as usize;
        let mut fast: usize = nums[nums[0] as usize] as usize;
        // util slow meet fast
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }

        fast = 0_usize;
        while slow != fast {
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }
        slow as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_287() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 5]), 5);
        assert_eq!(Solution::find_duplicate(vec![5, 1, 2, 3, 4, 5]), 5);
    }
}

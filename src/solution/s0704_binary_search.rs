/**
 * [792] Binary Search
 *
 * Given a sorted (in ascending order) integer array nums of n elements and a target value, write a function to search target in nums. If target exists, then return its index, otherwise return -1.
 *
 * <br />
 * Example 1:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * 	You may assume that all elements in nums are unique.
 * 	n will be in the range [1, 10000].
 * 	The value of each element in nums will be in the range [-9999, 9999].
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = (nums.len() as i32) - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Less => {
                    lo = mid + 1;
                }
                Ordering::Greater => {
                    hi = mid - 1;
                }
                Ordering::Equal => {
                    return mid;
                }
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_792() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![5], 6), -1);
        assert_eq!(Solution::search(vec![1, 2], 0), -1);
        assert_eq!(Solution::search(vec![1, 2], 1), 0);
        assert_eq!(Solution::search(vec![1, 2], 2), 1);
        assert_eq!(Solution::search(vec![1, 2], 3), -1);
    }
}

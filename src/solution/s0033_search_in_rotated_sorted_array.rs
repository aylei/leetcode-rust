/**
 * [33] Search in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 *
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 *
 * You may assume no duplicate exists in the array.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * Example 1:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 *
 */
pub struct Solution {}

/*
    \
       8
     7   9
   6       1
     5   2
       3 \
Consider the given array as ring, each time we split the ring and judge which part is the target belong to, then it's ordinary binary search.
*/

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        if size == 0 {
            return -1;
        }
        let mut base = 0_usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if nums[mid] == target {
                return mid as i32;
            }
            // we split the ring to [base..half] & [half+1..base-1]
            // if target not in [base..half] ring, move base to select another ring
            if !(((nums[base] < nums[mid]) && (target >= nums[base] && target <= nums[mid]))
                || ((nums[base] > nums[mid]) && (target >= nums[base] || target <= nums[mid])))
            {
                base = mid;
            }
            size -= half;
        }
        if nums[base] == target {
            base as i32
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                0
            ),
            9
        );
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                1006
            ),
            2
        );
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![], 3), -1);
    }
}

/**
 * [219] Contains Duplicate II
 *
 * Given an array of integers and an integer k, find out whether there are two distinct indices i and j in the array such that nums[i] = nums[j] and the absolute difference between i and j is at most k.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: nums = <span id="example-input-1-1">[1,2,3,1]</span>, k = <span id="example-input-1-2">3</span>
 * Output: <span id="example-output-1">true</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: nums = <span id="example-input-2-1">[1,0,1,1]</span>, k = <span id="example-input-2-2">1</span>
 * Output: <span id="example-output-2">true</span>
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: nums = <span id="example-input-3-1">[1,2,3,1,2,3]</span>, k = <span id="example-input-3-2">2</span>
 * Output: <span id="example-output-3">false</span>
 *
 * </div>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-ii/
// discuss: https://leetcode.com/problems/contains-duplicate-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            match map.get(&num) {
                Some(v) => {
                    if idx - v <= k as usize {
                        return true;
                    }
                    map.insert(num, idx);
                }
                None => {
                    map.insert(num, idx);
                }
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 3),
            true
        );
    }
}

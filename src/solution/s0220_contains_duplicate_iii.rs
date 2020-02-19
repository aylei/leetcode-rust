/**
 * [220] Contains Duplicate III
 *
 * Given an array of integers, find out whether there are two distinct indices i and j in the array such that the absolute difference between nums[i] and nums[j] is at most t and the absolute difference between i and j is at most k.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: nums = <span id="example-input-1-1">[1,2,3,1]</span>, k = <span id="example-input-1-2">3</span>, t = <span id="example-input-1-3">0</span>
 * Output: <span id="example-output-1">true</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: nums = <span id="example-input-2-1">[1,0,1,1]</span>, k = <span id="example-input-2-2">1</span>, t = <span id="example-input-2-3">2</span>
 * Output: <span id="example-output-2">true</span>
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: nums = <span id="example-input-3-1">[1,5,9,1,5,9]</span>, k = <span id="example-input-3-2">2</span>, t = <span id="example-input-3-3">3</span>
 * Output: <span id="example-output-3">false</span>
 *
 * </div>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-iii/
// discuss: https://leetcode.com/problems/contains-duplicate-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if k < 1 || t < 0 {
            return false;
        }
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let remap = nums[i] as i64 - i32::min_value() as i64;
            let bucket = remap / (t as i64 + 1);
            println!("{} {}", remap, bucket);
            if map.contains_key(&bucket)
                || map
                    .get(&(bucket - 1))
                    .map_or(false, |v| remap - v <= t as i64)
                || map
                    .get(&(bucket + 1))
                    .map_or(false, |v| v - remap <= t as i64)
            {
                return true;
            }
            if i >= k as usize {
                let last_bucket =
                    (nums[i - k as usize] as i64 - i32::min_value() as i64) / (t as i64 + 1);
                map.remove(&last_bucket);
            }
            map.insert(bucket, remap);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_220() {
        // assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,5,9,1,5,9], 2, 3), false);
        // assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,2,3,1], 3, 0), true);
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
            false
        );
    }
}

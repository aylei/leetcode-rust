use std::hash::Hash;

/**
 * [349] Intersection of Two Arrays
 *
 * Given two arrays, write a function to compute their intersection.
 *
 * Example 1:
 *
 *
 * Input: nums1 = <span id="example-input-1-1">[1,2,2,1]</span>, nums2 = <span id="example-input-1-2">[2,2]</span>
 * Output: <span id="example-output-1">[2]</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: nums1 = <span id="example-input-2-1">[4,9,5]</span>, nums2 = <span id="example-input-2-2">[9,4,9,8,4]</span>
 * Output: <span id="example-output-2">[9,4]</span>
 * </div>
 *
 * Note:
 *
 *
 * 	Each element in the result must be unique.
 * 	The result can be in any order.
 *
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let s1: HashSet<i32> = nums1.into_iter().collect();
        let s2: HashSet<i32> = nums2.into_iter().collect();
        let mut ret: HashSet<i32> = HashSet::new();
        for num1 in s1 {
            if s2.contains(&num1) {
                ret.insert(num1);
            }
        }
        ret.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_349() {}
}

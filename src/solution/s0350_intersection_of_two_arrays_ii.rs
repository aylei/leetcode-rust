/**
 * [350] Intersection of Two Arrays II
 *
 * Given two arrays, write a function to compute their intersection.
 *
 * Example 1:
 *
 *
 * Input: nums1 = <span id="example-input-1-1">[1,2,2,1]</span>, nums2 = <span id="example-input-1-2">[2,2]</span>
 * Output: <span id="example-output-1">[2,2]</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: nums1 = <span id="example-input-2-1">[4,9,5]</span>, nums2 = <span id="example-input-2-2">[9,4,9,8,4]</span>
 * Output: <span id="example-output-2">[4,9]</span>
 * </div>
 *
 * Note:
 *
 *
 * 	Each element in the result should appear as many times as it shows in both arrays.
 * 	The result can be in any order.
 *
 *
 * Follow up:
 *
 *
 * 	What if the given array is already sorted? How would you optimize your algorithm?
 * 	What if nums1's size is small compared to nums2's size? Which algorithm is better?
 * 	What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut nums1 = nums1;
        nums1.sort();
        let mut nums2 = nums2;
        nums2.sort();

        for num1 in nums1 {
            if let Ok(index) = nums2.binary_search(&num1) {
                ret.push(num1);
                nums2.remove(index);
            }
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_350() {}
}

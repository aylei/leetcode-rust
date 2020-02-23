/**
 * [303] Range Sum Query - Immutable
 *
 * Given an integer array nums, find the sum of the elements between indices i and j (i &le; j), inclusive.
 *
 * Example:<br>
 *
 * Given nums = [-2, 0, 3, -5, 2, -1]
 *
 * sumRange(0, 2) -> 1
 * sumRange(2, 5) -> -1
 * sumRange(0, 5) -> -3
 *
 *
 *
 * Note:<br>
 * <ol>
 * You may assume that the array does not change.
 * There are many calls to sumRange function.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-immutable/
// discuss: https://leetcode.com/problems/range-sum-query-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumArray {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut res = 0;
        let mut vec = Vec::with_capacity(nums.len());
        for &num in nums.iter() {
            res += num;
            vec.push(res);
        }
        NumArray { nums: vec }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (i, j) = (i as usize, j as usize);
        self.nums[j] - if i > 0 { self.nums[i - 1] } else { 0 }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_303() {
        let mut nums = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(nums.sum_range(0, 2), 1);
        assert_eq!(nums.sum_range(2, 5), -1);
        assert_eq!(nums.sum_range(0, 5), -3);
    }
}

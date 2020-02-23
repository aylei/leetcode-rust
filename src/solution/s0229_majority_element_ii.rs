/**
 * [229] Majority Element II
 *
 * Given an integer array of size n, find all elements that appear more than &lfloor; n/3 &rfloor; times.
 *
 * Note: The algorithm should run in linear time and in O(1) space.
 *
 * Example 1:
 *
 *
 * Input: [3,2,3]
 * Output: [3]
 *
 * Example 2:
 *
 *
 * Input: [1,1,1,3,3,2,2,2]
 * Output: [1,2]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element-ii/
// discuss: https://leetcode.com/problems/majority-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let (mut vote0, mut vote1, mut candidate0, mut candidate1) = (0, 0, -1, -2);
        for &num in nums.iter() {
            if num == candidate0 {
                vote0 += 1;
            } else if num == candidate1 {
                vote1 += 1;
            } else if vote0 == 0 {
                candidate0 = num;
                vote0 = 1;
            } else if vote1 == 0 {
                candidate1 = num;
                vote1 = 1;
            } else {
                vote0 -= 1;
                vote1 -= 1;
            }
        }
        // the presents of majority element is not guaranteed, we have to do a double check
        let mut res = Vec::new();
        for &v in vec![candidate0, candidate1].iter() {
            let mut count = 0;
            for &num in nums.iter() {
                if v == num {
                    count += 1;
                }
            }
            if count > (nums.len() / 3) as i32 {
                res.push(v);
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_229() {
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 2, 2, 2, 3, 3, 3]),
            vec![]
        );
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 2, 2, 3, 3, 3]),
            vec![1, 3]
        );
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![5, 6, 6]), vec![6]);
        assert_eq!(Solution::majority_element(vec![1, 2, 3, 4]), vec![]);
    }
}

/**
 * [41] First Missing Positive
 *
 * Given an unsorted integer array, find the smallest missing positive integer.
 *
 * Example 1:
 *
 *
 * Input: [1,2,0]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: [3,4,-1,1]
 * Output: 2
 *
 *
 * Example 3:
 *
 *
 * Input: [7,8,9,11,12]
 * Output: 1
 *
 *
 * Note:
 *
 * Your algorithm should run in O(n) time and uses constant extra space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-missing-positive/
// discuss: https://leetcode.com/problems/first-missing-positive/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        let mut i = 0;
        let mut c = 0;
        while i < len {
            let num = nums[i];
            if num > 0 && num - 1 < (len as i32) {
                c += 1;
                nums.swap((num - 1) as usize, i);
                if (num - 1) > (i as i32) && (num != nums[i]) {
                    continue;
                }
            }
            i += 1;
        }
        println!("{}", c);
        for (i, &num) in nums.iter().enumerate() {
            if num != ((i + 1) as i32) {
                return (i + 1) as i32;
            }
        }
        return (len + 1) as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41() {
        assert_eq!(Solution::first_missing_positive(vec![2, 2]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2]),
            1
        );
        assert_eq!(
            Solution::first_missing_positive(vec![2, 2, 2, 2, 2, 2, 2]),
            1
        );
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![2, 1, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![7, 8, 1, 2, 3, 3, 3, 3, 3, 3, 3, -5, -7, 1234]),
            4
        );
    }
}

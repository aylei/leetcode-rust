/**
 * [260] Single Number III
 *
 * Given an array of numbers nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once.
 *
 * Example:
 *
 *
 * Input:  [1,2,1,3,2,5]
 * Output: [3,5]
 *
 * Note:
 *
 * <ol>
 * 	The order of the result is not important. So in the above example, [5, 3] is also correct.
 * 	Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?
 * </ol>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-iii/
// discuss: https://leetcode.com/problems/single-number-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut res = 0;
        for &num in nums.iter() {
            res = res ^ num;
        }
        let right_most_set_bit = res & !(res - 1);
        let mut bit_set = 0;
        let mut bit_unset = 0;
        for &num in nums.iter() {
            if num & right_most_set_bit == 0 {
                bit_unset = bit_unset ^ num;
            } else {
                bit_set = bit_set ^ num;
            }
        }
        return vec![bit_set, bit_unset];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_260() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 2, 3, 4]), vec![3, 4]);
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    }
}

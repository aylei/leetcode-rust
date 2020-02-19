/**
 * [66] Plus One
 *
 * Given a non-empty array of digits representing a non-negative integer, plus one to the integer.
 *
 * The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.
 *
 * You may assume the integer does not contain any leading zero, except the number 0 itself.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 *
 *
 * Example 2:
 *
 *
 * Input: [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/plus-one/
// discuss: https://leetcode.com/problems/plus-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 0;
        for i in (0..digits.len()).rev() {
            digits[i] = if digits[i] == 9 {
                carry = 1;
                0
            } else {
                carry = 0;
                digits[i] + 1
            };
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            digits.insert(0, 1);
        }
        digits
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
        assert_eq!(
            Solution::plus_one(vec![1, 0, 9, 9, 9, 9]),
            vec![1, 1, 0, 0, 0, 0]
        );
    }
}

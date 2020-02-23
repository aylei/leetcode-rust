/**
 * [179] Largest Number
 *
 * Given a list of non negative integers, arrange them such that they form the largest number.
 *
 * Example 1:
 *
 *
 * Input: [10,2]
 * Output: "210"
 *
 * Example 2:
 *
 *
 * Input: [3,30,34,5,9]
 * Output: "9534330"
 *
 *
 * Note: The result may be very large, so you need to return a string instead of an integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-number/
// discuss: https://leetcode.com/problems/largest-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
        if nums[0] == "0" {
            return "0".to_owned();
        }
        nums.iter().fold(String::new(), |mut s, num| {
            s.push_str(num);
            s
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_179() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_owned()
        );
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121".to_owned());
    }
}

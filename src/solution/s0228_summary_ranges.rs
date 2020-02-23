/**
 * [228] Summary Ranges
 *
 * Given a sorted integer array without duplicates, return the summary of its ranges.
 *
 * Example 1:
 *
 *
 * Input:  [0,1,2,4,5,7]
 * Output: ["0->2","4->5","7"]
 * Explanation: 0,1,2 form a continuous range; 4,5 form a continuous range.
 *
 *
 * Example 2:
 *
 *
 * Input:  [0,2,3,4,6,8,9]
 * Output: ["0","2->4","6","8->9"]
 * Explanation: 2,3,4 form a continuous range; 8,9 form a continuous range.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/summary-ranges/
// discuss: https://leetcode.com/problems/summary-ranges/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut res = Vec::new();
        let mut curr = nums[0];
        let mut start = nums[0];
        for num in nums.into_iter().skip(1) {
            if num == curr + 1 {
                curr = num;
            } else {
                // seq done
                Solution::record(&mut res, start, curr);
                start = num;
                curr = num;
            }
        }
        Solution::record(&mut res, start, curr);
        res
    }

    pub fn record(vec: &mut Vec<String>, start: i32, end: i32) {
        if start == end {
            vec.push(format!("{}", start))
        } else {
            vec.push(format!("{}->{}", start, end))
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_228() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 3, 4, 5, 6]),
            vec_string!["0->6"]
        );
    }
}

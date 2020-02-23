/**
 * [239] Sliding Window Maximum
 *
 * Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position. Return the max sliding window.
 *
 * Example:
 *
 *
 * Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation:
 *
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * Note: <br />
 * You may assume k is always valid, 1 &le; k &le; input array's size for non-empty array.
 *
 * Follow up:<br />
 * Could you solve it in linear time?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-window-maximum/
// discuss: https://leetcode.com/problems/sliding-window-maximum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut deq: VecDeque<(usize, i32)> = VecDeque::new();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            // maintain sliding window
            if !deq.is_empty() && (*deq.front().unwrap()).0 as i32 <= (i as i32) - k {
                deq.pop_front();
                let mut max = i32::min_value();
                let mut count = 0_usize;
                for (j, &v) in deq.iter().enumerate() {
                    if v.1 >= max {
                        max = v.1;
                        count = j;
                    }
                }
                for _ in 0..count {
                    deq.pop_front();
                }
            }
            let num = nums[i];
            if !deq.is_empty() && (*deq.front().unwrap()).1 <= num {
                while !deq.is_empty() {
                    deq.pop_front();
                }
            }
            deq.push_back((i, num));
            if i + 1 >= k as usize {
                res.push((*deq.front().unwrap()).1);
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
    fn test_239() {
        assert_eq!(
            Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
            vec![10, 10, 9, 2]
        );
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
            vec![3, 3, 2, 5]
        );
        assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}

/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 *
 * Note:
 *
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 *
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut previous = nums[0] - 1;
        while i < len - 2 {
            // skip same number
            if nums[i] == previous {
                i += 1;
                continue;
            }
            previous = nums[i];
            let mut vec = Solution::two_sum(&nums[(i + 1)..len], 0 - nums[i]);
            for t in vec.iter() {
                result.push(vec![nums[i], t.0, t.1]);
            }
            i += 1;
        }
        result
    }

    // 2 sum using 2 pointers: nums[0] ->   <- nums[len-1]
    #[inline(always)]
    fn two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
        let (mut i, mut j) = (0_usize, nums.len() - 1);
        let mut result = Vec::new();
        while i < j {
            if nums[i] + nums[j] < sum {
                i += 1
            } else if nums[i] + nums[j] > sum {
                j -= 1
            } else {
                result.push((nums[i], nums[j]));
                i = Solution::next_unique(nums, i, true);
                j = Solution::next_unique(nums, j, false);
            }
        }
        result
    }

    // seek next un-repeat number
    #[inline(always)]
    fn next_unique(nums: &[i32], idx: usize, forward: bool) -> usize {
        let curr = nums[idx];
        let mut i = idx;
        while i > 0 && i < nums.len() && nums[i] == curr {
            i = if forward { i + 1 } else { i - 1 }
        }
        i
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![
                -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4,
                2, -5, -4, 4, 7
            ]),
            vec![
                vec![-10, 2, 8],
                vec![-10, 3, 7],
                vec![-10, 4, 6],
                vec![-10, 5, 5],
                vec![-9, 2, 7],
                vec![-9, 3, 6],
                vec![-9, 4, 5],
                vec![-8, 2, 6],
                vec![-8, 3, 5],
                vec![-8, 4, 4],
                vec![-7, -1, 8],
                vec![-7, 2, 5],
                vec![-7, 3, 4],
                vec![-6, -2, 8],
                vec![-6, -1, 7],
                vec![-6, 2, 4],
                vec![-5, -3, 8],
                vec![-5, -2, 7],
                vec![-5, -1, 6],
                vec![-5, 2, 3],
                vec![-4, -4, 8],
                vec![-4, -3, 7],
                vec![-4, -2, 6],
                vec![-4, -1, 5],
                vec![-3, -2, 5],
                vec![-3, -1, 4],
                vec![-2, -1, 3],
                vec![-1, -1, 2]
            ]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty_vec);
    }
}

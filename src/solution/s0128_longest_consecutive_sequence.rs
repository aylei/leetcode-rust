/**
 * [128] Longest Consecutive Sequence
 *
 * Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
 *
 * Your algorithm should run in O(n) complexity.
 *
 * Example:
 *
 *
 * Input: [100, 4, 200, 1, 3, 2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-consecutive-sequence/
// discuss: https://leetcode.com/problems/longest-consecutive-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
要找到连续子串, 基本策略就是对每个 num, 判断 num+1, num+2, num+3... 是否在数组中, 直到不再连续

工程里写的话用排序是最清晰可维护的(需求变了很好改), 排序之后查找 num+1 是否存在就只需要 O(1) 的复杂度了:
看下一个元素是不是 num+1 即可

但题目一定要求 O(N) 的解法, 只能想些奇怪的办法了, HashSet 也能达到 O(1) 的查找效率. 但假如对每个元素
都做一遍, 最差就是 O(N^2) 了, 可以发现对于一个连续序列 1,2,3,4,5,6 我们从 1 开始查就能找到这个序列,
从 2,3,4,5,6 开始查都是在做重复计算, 因此对于一个 num, 假如 num-1 存在于 HashSet 中, 我们就不需要考虑
它了, 因为它是一次重复的计算.
*/
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let nums = nums.into_iter().collect::<Vec<_>>();
        for &num in nums.iter() {
            if !nums.contains(&(num - 1)) {
                let mut curr = num;
                let mut curr_max = 1;
                while nums.contains(&(curr + 1)) {
                    curr += 1;
                    curr_max += 1;
                }
                max = i32::max(curr_max, max);
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4)
    }
}

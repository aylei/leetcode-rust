/**
 * [169] Majority Element
 *
 * Given an array of size n, find the majority element. The majority element is the element that appears more than &lfloor; n/2 &rfloor; times.
 *
 * You may assume that the array is non-empty and the majority element always exist in the array.
 *
 * Example 1:
 *
 *
 * Input: [3,2,3]
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: [2,2,1,1,1,2,2]
 * Output: 2
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element/
// discuss: https://leetcode.com/problems/majority-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
抄的题解：Boyer-Moore Voting Algorithm
自己只能想到 HashMap 和排序, 真是太鸡儿菜了...

Boyer-Moore Voting Algorithm 的思路是假设当前值为主元素, 碰到当前值则 +1, 非当前值则 -1, 计数器一旦归零,
就取下一个数为主元素

最后留下的数一定主元素

证明也很简单, 假设我们从第 i 位开始选择了一个数 A, 并且这个数 A 保持到了循环终止, 那么:

我们知道, 第 nums[i..n] 中, A 是主元素, nums[0..i] 中, 有一个数 B 出现了一半的次数

假如 A = B, 那么 A 出现了大于一半的次数, A 一定是主元素

假如 A != B, 且主元素不是 A, 那么 B 包括其他任何数在整个数组中出现的次数一定不到一半(因为 B 包括其他任何数
在前半部分**至多**出现一半, 而在后半部分不到一半), 因此不存在主元素, 这与题目给定的"一定存在主元素"矛盾, 因此
A 一定是主元素
*/

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for &num in nums.iter() {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }
        candidate
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_169() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}

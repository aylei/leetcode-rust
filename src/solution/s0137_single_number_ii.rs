/**
 * [137] Single Number II
 *
 * Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.
 *
 * Note:
 *
 * Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
 *
 * Example 1:
 *
 *
 * Input: [2,2,3,2]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: [0,1,0,1,0,1,99]
 * Output: 99
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-ii/
// discuss: https://leetcode.com/problems/single-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
糅合了一下 https://leetcode.com/problems/single-number-ii/discuss/43296/An-General-Way-to-Handle-All-this-sort-of-questions. 和  https://leetcode.com/problems/single-number-ii/discuss/43294/Challenge-me-thx

第一个链接给出了通用解法: 对于一个数出现 M 次其它数都出现了 K 的场景, 我们可以用位运算记录 K 种状态(作为一个计数器)来解

这题的真值表(3种状态使用2位):

a b   c/c   a'b'/a'b'
0 0   1/0   0 1 /0 0
0 1   1/0   1 0 /0 1
1 0   1/0   0 0 /1 0

根据数电的知识, 要根据这个真值表写出逻辑表达式, 以输出端为 '1' 的结果为准, 将每行的输入变量写成 AND 形式, 其中为 0 的输入量需要取反, 再将这几个 AND 形式做 OR 即可

令 a' = 1, 则:

a b c a'
0 1 1 1    ~a & b & c
1 0 0 1    a & ~b & ~c

a' = (~a & b & c) | (a & ~b & ~c)

同理:

b' = (~a & b & ~c) | (~a & ~b & c)

这个每轮计算的位次数达到 17 次, 可以再优化一下:

对 b' 化简: b' = ~a & (b & ~c | ~b & c) = ~a & b ^ c

但这时 a 仍然比较复杂, 我们可以考虑能否用每轮算出的 b' 来简化 a 的计算, 则:

a (b) b' c a' b'
1 (0) 0  0 1  0
0 (1) 0  1 1  0

重写一下就是 a' = (a & ~b' & ~c) | (~a & ~b' & c) = ~b' & (a & ~c | ~a & c) = ~b' & a ^ c

这个就和最开始第二链接里给出的超简洁解法一致了

最后的话, a 或 b 为 1 都可以输出到 1 (目标数出现1次或出现2次), 输出 a | b 即可
*/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for &num in nums.iter() {
            b = !a & (b ^ num);
            a = !b & (a ^ num);
        }
        return a | b;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_137() {
        assert_eq!(Solution::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);
    }
}

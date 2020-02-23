/**
 * [89] Gray Code
 *
 * The gray code is a binary numeral system where two successive values differ in only one bit.
 *
 * Given a non-negative integer n representing the total number of bits in the code, print the sequence of gray code. A gray code sequence must begin with 0.
 *
 * Example 1:
 *
 *
 * Input: 2
 * Output: [0,1,3,2]
 * Explanation:
 * 00 - 0
 * 01 - 1
 * 11 - 3
 * 10 - 2
 *
 * For a given n, a gray code sequence may not be uniquely defined.
 * For example, [0,2,3,1] is also a valid gray code sequence.
 *
 * 00 - 0
 * 10 - 2
 * 11 - 3
 * 01 - 1
 *
 *
 * Example 2:
 *
 *
 * Input: 0
 * Output: [0]
 * Explanation: We define the gray code sequence to begin with 0.
 *              A gray code sequence of n has size = 2^n, which for n = 0 the size is 2^0 = 1.
 *              Therefore, for n = 0 the gray code sequence is [0].
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/gray-code/
// discuss: https://leetcode.com/problems/gray-code/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
0000
0001 <- flip [0] to 1, traverse [] in reverse order
0011 <- flip [1] to 1, traverse [0] in reverse order
0010
0110 <- flip [2] to 1, traverse [1,0] in reverse order
0111
0101
0100
1100 <- flip [3] to 1, traverse [2,1,0] in reverse order
 */
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![0];
        for i in 0..n {
            for j in (0..res.len()).rev() {
                res.push(2_i32.pow(i as u32) + res[j]);
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
    fn test_89() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
        assert_eq!(Solution::gray_code(0), vec![0]);
        assert_eq!(Solution::gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}

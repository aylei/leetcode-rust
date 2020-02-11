/**
 * [343] Integer Break
 *
 * Given a positive integer n, break it into the sum of at least two positive integers and maximize the product of those integers. Return the maximum product you can get.
 *
 * Example 1:
 *
 * <div>
 *
 * Input: <span id="example-input-1-1">2</span>
 * Output: <span id="example-output-1">1</span>
 * Explanation: 2 = 1 + 1, 1 &times; 1 = 1.
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">10</span>
 * Output: <span id="example-output-2">36</span>
 * Explanation: 10 = 3 + 3 + 4, 3 &times; 3 &times; 4 = 36.
 *
 * Note: You may assume that n is not less than 2 and not larger than 58.
 * </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        // dp
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let mut dp = vec![0; (n + 1) as usize];
        // when used as factor, no need to break to addends
        dp[2] = 2;
        dp[3] = 3;
        for i in 4..(n + 1) {
            Self::helper(&mut dp, i);
        }
        dp[n as usize]
    }

    fn helper(dp: &mut Vec<i32>, n: i32) {
        let mut num1: usize = 2;
        let mut num2: usize = n as usize - 2;
        let mut res = 0;
        while num1 <= num2 {
            res = res.max(dp[num1] * dp[num2]);
            num1 += 1;
            num2 -= 1;
        }
        dp[n as usize] = res;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_343() {
        println!("{}", Solution::integer_break(10));
    }
}

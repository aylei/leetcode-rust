/**
 * [441] Arranging Coins
 *
 * You have a total of n coins that you want to form in a staircase shape, where every k-th row must have exactly k coins.
 *
 * Given n, find the total number of full staircase rows that can be formed.
 *
 * n is a non-negative integer and fits within the range of a 32-bit signed integer.
 *
 * Example 1:
 *
 * n = 5
 *
 * The coins can form the following rows:
 * ¤
 * ¤ ¤
 * ¤ ¤
 *
 * Because the 3rd row is incomplete, we return 2.
 *
 *
 *
 * Example 2:
 *
 * n = 8
 *
 * The coins can form the following rows:
 * ¤
 * ¤ ¤
 * ¤ ¤ ¤
 * ¤ ¤
 *
 * Because the 4th row is incomplete, we return 3.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // x(x + 1) / 2 + reminder = n
        // binary search
        let mut left = 0;
        let mut right = 100_000;
        while left <= right {
            let mid = left + (right - left) / 2;
            let curr: i64 = mid * (mid + 1) / 2;
            let reminder = n as i64 - curr;
            if reminder >= mid + 1 {
                left = mid + 1;
            } else if reminder < 0 {
                right = mid - 1;
            } else {
                return mid as i32;
            }
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_441() {
        Solution::arrange_coins(5);
    }
}

/**
 * [367] Valid Perfect Square
 *
 * Given a positive integer num, write a function which returns True if num is a perfect square else False.
 *
 * Note: Do not use any built-in library function such as sqrt.
 *
 * Example 1:
 *
 * <div>
 *
 * Input: <span id="example-input-1-1">16</span>
 * Output: <span id="example-output-1">true</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">14</span>
 * Output: <span id="example-output-2">false</span>
 *
 * </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 0 || num == 1 {
            return true;
        }
        // binary search
        let (mut left, mut right) = (0, 100_000);
        while left <= right {
            let mid = left + (right - left) / 2;
            if mid == num / mid {
                return mid * mid == num;
            } else if mid > num / mid {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_367() {
        println!("{}", Solution::is_perfect_square(1));
    }
}

/**
 * [69] Sqrt(x)
 *
 * Implement int sqrt(int x).
 * 
 * Compute and return the square root of x, where x is guaranteed to be a non-negative integer.
 * 
 * Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.
 * 
 * Example 1:
 * 
 * 
 * Input: 4
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since 
 *              the decimal part is truncated, 2 is returned.
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

// Newton-Raphson for: root^2 - n = 0
// Tangent equation: y = 2 * root * x - (root^2 + n)
// Zero point: (root^2 + n) / (2 * root)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 { return x }
        let mut root = x as f64;
        loop {
            let new_root: f64 = (root * root + x as f64) / (2.0 * root);
            if root - new_root < 0.5 {
                root = new_root;
                break;
            }
            root = new_root;
        }
        root.trunc() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(16), 4);
        assert_eq!(Solution::my_sqrt(17), 4);
        assert_eq!(Solution::my_sqrt(81), 9);
        assert_eq!(Solution::my_sqrt(82), 9);
        assert_eq!(Solution::my_sqrt(100480576), 10024);
        assert_eq!(Solution::my_sqrt(100480577), 10024);
        assert_eq!(Solution::my_sqrt(100480575), 10023);
        assert_eq!(Solution::my_sqrt(100480575), 10023);
        assert_eq!(Solution::my_sqrt(80), 8);
        assert_eq!(Solution::my_sqrt(2), 1);
    }
}

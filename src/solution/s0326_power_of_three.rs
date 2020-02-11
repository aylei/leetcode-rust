/**
 * [326] Power of Three
 *
 * Given an integer, write a function to determine if it is a power of three.
 *
 * Example 1:
 *
 *
 * Input: 27
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: 0
 * Output: false
 *
 * Example 3:
 *
 *
 * Input: 9
 * Output: true
 *
 * Example 4:
 *
 *
 * Input: 45
 * Output: false
 *
 * Follow up:<br />
 * Could you do it without using any loop / recursion?
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        if n == 1 {
            return true;
        }
        n % 3 == 0 && Self::is_power_of_three(n / 3)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_326() {}
}

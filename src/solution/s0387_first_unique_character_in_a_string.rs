/**
 * [387] First Unique Character in a String
 *
 *
 * Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.
 *
 * Examples:
 *
 * s = "leetcode"
 * return 0.
 *
 * s = "loveleetcode",
 * return 2.
 *
 *
 *
 *
 * Note: You may assume the string contain only lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut lookup = [0; 256];
        for c in s.chars() {
            lookup[c as usize] = lookup[c as usize] + 1;
        }
        for (i, c) in s.chars().enumerate() {
            if lookup[c as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_387() {}
}

/**
 * [409] Longest Palindrome
 *
 * Given a string which consists of lowercase or uppercase letters, find the length of the longest palindromes that can be built with those letters.
 *
 * This is case sensitive, for example "Aa" is not considered a palindrome here.
 *
 * Note:<br />
 * Assume the length of given string will not exceed 1,010.
 *
 *
 * Example:
 *
 * Input:
 * "abccccdd"
 *
 * Output:
 * 7
 *
 * Explanation:
 * One longest palindrome that can be built is "dccaccd", whose length is 7.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut lookup = vec![0; 256];
        for c in s.chars() {
            lookup[c as usize] = lookup[c as usize] + 1;
        }
        let mut ret = 0;
        let mut extra = 0;
        for i in lookup {
            if i % 2 == 1 {
                extra = 1;
            }
            ret += i / 2 * 2;
        }
        ret + extra
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_409() {
        println!("{}", Solution::longest_palindrome("abccccdd".to_string()));
    }
}

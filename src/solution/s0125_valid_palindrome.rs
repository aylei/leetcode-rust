/**
 * [125] Valid Palindrome
 *
 * Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
 *
 * Note: For the purpose of this problem, we define empty string as valid palindrome.
 *
 * Example 1:
 *
 *
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "race a car"
 * Output: false
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome/
// discuss: https://leetcode.com/problems/valid-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let seq = s.chars().collect::<Vec<_>>();
        let (mut i, mut j) = (0_usize, seq.len() - 1);
        while i < j {
            while i < seq.len() && !seq[i].is_ascii_alphanumeric() {
                i += 1;
            }
            while j > 0 && !seq[j].is_ascii_alphanumeric() {
                j -= 1;
            }
            if i >= j {
                break;
            }
            if seq[i].to_ascii_lowercase() != seq[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
        assert_eq!(Solution::is_palindrome("0P".to_owned()), false);
    }
}

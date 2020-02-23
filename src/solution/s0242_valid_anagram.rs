/**
 * [242] Valid Anagram
 *
 * Given two strings s and t , write a function to determine if t is an anagram of s.
 *
 * Example 1:
 *
 *
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "rat", t = "car"
 * Output: false
 *
 *
 * Note:<br />
 * You may assume the string contains only lowercase alphabets.
 *
 * Follow up:<br />
 * What if the inputs contain unicode characters? How would you adapt your solution to such case?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-anagram/
// discuss: https://leetcode.com/problems/valid-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        Solution::hit(s) == Solution::hit(t)
    }

    fn hit(s: String) -> Vec<i32> {
        let mut hit = vec![0; 27];
        for ch in s.chars() {
            hit[(ch as u8 - 'a' as u8) as usize] += 1;
        }
        hit
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        );
    }
}

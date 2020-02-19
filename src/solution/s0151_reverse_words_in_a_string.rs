/**
 * [151] Reverse Words in a String
 *
 * Given an input string, reverse the string word by word.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: "the sky is blue"
 * Output: "blue is sky the"
 *
 *
 * Example 2:
 *
 *
 * Input: "  hello world!  "
 * Output: "world! hello"
 * Explanation: Your reversed string should not contain leading or trailing spaces.
 *
 *
 * Example 3:
 *
 *
 * Input: "a good   example"
 * Output: "example good a"
 * Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	A word is defined as a sequence of non-space characters.
 * 	Input string may contain leading or trailing spaces. However, your reversed string should not contain leading or trailing spaces.
 * 	You need to reduce multiple spaces between two words to a single space in the reversed string.
 *
 *
 *  
 *
 * Follow up:
 *
 * For C programmers, try to solve it in-place in O(1) extra space.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-words-in-a-string/
// discuss: https://leetcode.com/problems/reverse-words-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        let mut seq = s.trim().chars().collect::<Vec<_>>();
        seq.reverse();
        let mut start_idx = 0_usize;
        let mut i = 0_usize;
        while i < seq.len() {
            if seq[i] == ' ' {
                if i == start_idx {
                    seq.remove(i);
                    continue;
                }
                seq[start_idx..i].reverse();
                start_idx = i + 1;
            }
            i += 1;
        }
        seq[start_idx..].reverse();
        seq.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("  hello world!  ".to_owned()),
            "world! hello".to_owned()
        );
    }
}

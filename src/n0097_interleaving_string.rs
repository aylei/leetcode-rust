/**
 * [97] Interleaving String
 *
 * Given s1, s2, s3, find whether s3 is formed by the interleaving of s1 and s2.
 * 
 * Example 1:
 * 
 * 
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Solution::backtrack(&s1, &s2, &s3)
    }

    fn backtrack(s1: &str, s2: &str, s3: &str) -> bool {
        if s3.len() < 1 && s1.len() < 1 && s2.len() < 1 {
            return true;
        }
        if s1.len() > 0 && s3.len() > 0 && &s1[0..1] == &s3[0..1] {
            if Solution::backtrack(&s1[1..], s2, &s3[1..]) {
                return true;
            }
        }
        if s2.len() > 0 && s3.len() > 0 && &s2[0..1] == &s3[0..1] {
            if Solution::backtrack(s1, &s2[1..], &s3[1..]) {
                return true;
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
    fn test_97() {
        assert_eq!(Solution::is_interleave("aabcc".to_owned(), "dbbca".to_owned(), "aadbbcbcac".to_owned()), true);
        assert_eq!(Solution::is_interleave("aabcc".to_owned(), "dbbca".to_owned(), "aadbbbaccc".to_owned()), false);
        assert_eq!(Solution::is_interleave("a".to_owned(), "b".to_owned(), "a".to_owned()), false);
    }
}

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
        let mut cache = vec![vec![false;s2.len()+1];s1.len()+1];
        Solution::dfs(&s1.chars().collect(),
                      &s2.chars().collect(),
                      &s3.chars().collect(), 0, 0, 0, &mut cache)
    }

    fn dfs(s1: &Vec<char>, s2: &Vec<char>, s3: &Vec<char>, i: usize, j: usize, k: usize, invalid: &mut Vec<Vec<bool>>) -> bool {
        if invalid[i][j] { return false }
        if i == s1.len() && j == s2.len() && k == s3.len() { return true }
        let valid =
            (i < s1.len() && k < s3.len() && s1[i] == s3[k] && Solution::dfs(s1,s2,s3,i+1,j,k+1,invalid)) ||
            (j < s2.len() && k < s3.len() && s2[j] == s3[k] && Solution::dfs(s1,s2,s3,i,j+1,k+1,invalid));
        if !valid { invalid[i][j] = true }
        valid
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

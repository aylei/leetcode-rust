/**
 * [49] Group Anagrams
 *
 * Given an array of strings, group anagrams together.
 *
 * Example:
 *
 *
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 *
 * Note:
 *
 *
 * 	All inputs will be in lowercase.
 * 	The order of your output does not matter.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs.into_iter() {
            let mut key = [0; 26];
            for ch in s.chars() {
                key[(ch as u32 - 'a' as u32) as usize] += 1;
            }
            map.entry(key).or_insert(Vec::new()).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    // TODO: implement arbitrary match macro
    #[test]
    #[ignore]
    fn test_49() {
        assert_eq!(
            Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]),
            vec![
                vec_string!["tan", "nat"],
                vec_string!["bat"],
                vec_string!["eat", "ate", "tea"],
            ]
        );
    }
}

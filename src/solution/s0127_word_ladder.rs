/**
 * [127] Word Ladder
 *
 * Given two words (beginWord and endWord), and a dictionary's word list, find the length of shortest transformation sequence from beginWord to endWord, such that:
 *
 * <ol>
 * 	Only one letter can be changed at a time.
 * 	Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * </ol>
 *
 * Note:
 *
 *
 * 	Return 0 if there is no such transformation sequence.
 * 	All words have the same length.
 * 	All words contain only lowercase alphabetic characters.
 * 	You may assume no duplicates in the word list.
 * 	You may assume beginWord and endWord are non-empty and are not the same.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 *
 * Output: 5
 *
 * Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog",
 * return its length 5.
 *
 *
 * Example 2:
 *
 *
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 *
 * Output: 0
 *
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-ladder/
// discuss: https://leetcode.com/problems/word-ladder/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let len = word_list.len();
        let target = word_list.iter().position(|s| s == &end_word);
        if target.is_none() {
            return 0;
        }
        let target = target.unwrap();
        let mut deq = VecDeque::new();
        let mut distance = vec![0; len];
        let mut remain = (0..len).collect::<HashSet<_>>();
        deq.push_back(target);
        remain.remove(&target);
        while let Some(i) = deq.pop_front() {
            if Solution::connect(&begin_word, &word_list[i]) {
                return distance[i] + 2;
            }
            remain.retain(|&j| {
                if Solution::connect(&word_list[i], &word_list[j]) {
                    distance[j] = distance[i] + 1;
                    deq.push_back(j);
                    false
                } else {
                    true
                }
            });
        }
        0
    }

    #[inline(always)]
    fn connect(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut iter1 = s1.chars().into_iter();
        let mut iter2 = s2.chars().into_iter();
        let mut diff = 0;
        while let (Some(c1), Some(c2)) = (iter1.next(), iter2.next()) {
            if c1 != c2 {
                diff += 1;
                if diff >= 2 {
                    return false;
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_127() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec_string!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec_string!["hot", "dot", "dog", "lot", "log"]
            ),
            0
        );
    }
}

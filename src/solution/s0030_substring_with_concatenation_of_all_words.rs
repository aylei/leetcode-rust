/**
 * [30] Substring with Concatenation of All Words
 *
 * You are given a string, s, and a list of words, words, that are all of the same length. Find all starting indices of substring(s) in s that is a concatenation of each word in words exactly once and without any intervening characters.
 *
 * Example 1:
 *
 *
 * Input:
 *   s = "barfoothefoobarman",
 *   words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoor" and "foobar" respectively.
 * The output order does not matter, returning [9,0] is fine too.
 *
 *
 * Example 2:
 *
 *
 * Input:
 *   s = "wordgoodgoodgoodbestword",
 *   words = ["word","good","best","word"]
 * Output: []
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// discuss: https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
struct Term {
    expect: i32,
    count: i32,
}
impl Term {
    fn new(expect: i32, count: i32) -> Self {
        Term { expect, count }
    }
    fn inc_expect(&mut self) {
        self.expect += 1;
    }
    fn inc(&mut self) {
        self.count += 1;
    }
    fn dec(&mut self) {
        self.count -= 1;
    }
    fn exhausted(&self) -> bool {
        self.count > self.expect
    }
    fn reset(&mut self) {
        self.count = 0;
    }
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() < 1 {
            return vec![];
        }
        let word_len = words[0].len();
        if word_len < 1 {
            return vec![];
        }
        let substr_len = word_len * words.len();
        let mut map: HashMap<&str, Term> = HashMap::with_capacity(words.len());
        for word in words.iter() {
            map.entry(word).or_insert(Term::new(0, 0)).inc_expect();
        }
        let mut result: Vec<i32> = Vec::new();
        // we can split terms in N ways, where N = word_len
        for shift in 0..word_len {
            let mut i = shift;
            let mut j = shift;
            // we do a sliding window for each round
            while j + word_len - 1 < s.len() {
                match map.entry(&s[j..j + word_len]) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().inc();
                        // term exhausted, shrink the window to release
                        if entry.get().exhausted() {
                            while i < j {
                                let term = &s[i..i + word_len];
                                map.entry(term).and_modify(|t| t.dec());
                                i += word_len;
                                if term == &s[j..j + word_len] {
                                    break;
                                }
                            }
                            j += word_len;
                        } else {
                            if j - i < (words.len() - 1) * word_len {
                                j += word_len;
                            } else {
                                // matched!
                                result.push(i as i32);
                                // move the whole window, release the dropped term
                                map.entry(&s[i..i + word_len]).and_modify(|t| t.dec());
                                j += word_len;
                                i += word_len;
                            }
                        }
                    }
                    // bad term, move over and do a reset
                    Entry::Vacant(entry) => {
                        map.iter_mut().for_each(|(_, v)| v.reset());
                        j += word_len;
                        i = j;
                    }
                }
            }
            map.iter_mut().for_each(|(_, v)| v.reset())
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_30() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![10]
        );
    }
}

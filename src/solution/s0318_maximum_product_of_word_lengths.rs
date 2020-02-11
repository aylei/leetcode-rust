/**
 * [318] Maximum Product of Word Lengths
 *
 * Given a string array words, find the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. You may assume that each word will contain only lower case letters. If no such two words exist, return 0.
 *
 * Example 1:
 *
 *
 * Input: ["abcw","baz","foo","bar","xtfn","abcdef"]
 * Output: 16
 * Explanation: The two words can be "abcw", "xtfn"<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">.</span>
 *
 * Example 2:
 *
 *
 * Input: ["a","ab","abc","d","cd","bcd","abcd"]
 * Output: 4
 * Explanation: The two words can be "ab", "cd"<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">.</span>
 *
 * Example 3:
 *
 *
 * Input: ["a","aa","aaa","aaaa"]
 * Output: 0
 * Explanation: No such pair of words.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut ret = 0;
        // use a int value to store word characters
        // if a word contains 'a', then the resulting int is b'00000000000000000000000000000001'
        // if it also contains 'b', then the resulting int is b'00000000000000000000000000000011' and so on
        let mut values = vec![0; words.len()];
        for (i, word) in words.iter().enumerate() {
            for c in word.as_bytes() {
                values[i] |= 1 << (c - b'a');
            }
        }
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if values[i] & values[j] == 0 && words[i].len() * words[j].len() > ret {
                    ret = words[i].len() * words[j].len();
                }
            }
        }
        ret as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_318() {}
}

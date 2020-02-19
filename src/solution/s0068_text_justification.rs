/**
 * [68] Text Justification
 *
 * Given an array of words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
 *
 * You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
 *
 * Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
 *
 * For the last line of text, it should be left justified and no extra space is inserted between words.
 *
 * Note:
 *
 *
 * 	A word is defined as a character sequence consisting of non-space characters only.
 * 	Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
 * 	The input array words contains at least one word.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * words = ["This", "is", "an", "example", "of", "text", "justification."]
 * maxWidth = 16
 * Output:
 * [
 *    "This    is    an",
 *    "example  of text",
 *    "justification.  "
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * words = ["What","must","be","acknowledgment","shall","be"]
 * maxWidth = 16
 * Output:
 * [
 *   "What   must   be",
 *   "acknowledgment  ",
 *   "shall be        "
 * ]
 * Explanation: Note that the last line is "shall be    " instead of "shall     be",
 *              because the last line must be left-justified instead of fully-justified.
 *              Note that the second line is also left-justified becase it contains only one word.
 *
 *
 * Example 3:
 *
 *
 * Input:
 * words = ["Science","is","what","we","understand","well","enough","to","explain",
 *          "to","a","computer.","Art","is","everything","else","we","do"]
 * maxWidth = 20
 * Output:
 * [
 *   "Science  is  what we",
 *   "understand      well",
 *   "enough to explain to",
 *   "a  computer.  Art is",
 *   "everything  else  we",
 *   "do                  "
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/text-justification/
// discuss: https://leetcode.com/problems/text-justification/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = Vec::new();
        let max_width = max_width as usize;
        let mut i = 0;
        let mut row_len = 0;
        let mut buf = Vec::new();
        while i < words.len() {
            if words[i].len() > max_width {
                unreachable!()
            }
            let old_len = row_len;
            row_len += words[i].len() + if row_len > 0 { 1 } else { 0 };
            if row_len > max_width {
                res.push(Solution::compact(buf, max_width, old_len));
                buf = Vec::new();
                row_len = 0;
            } else {
                buf.push(words[i].clone());
                i += 1;
            }
        }
        res.push(Solution::compact_last(buf, max_width));
        res
    }

    fn compact(words: Vec<String>, max_width: usize, row_len: usize) -> String {
        let spaces = max_width - (row_len - words.len() + 1);
        let avg_spaces = spaces / usize::max(1, words.len() - 1);
        let mut extra_spaces = spaces - avg_spaces * usize::max(1, words.len() - 1);;
        let mut res = String::new();
        for (i, word) in words.iter().enumerate() {
            res.push_str(word);
            if words.len() < 2 || (i < words.len() - 1) {
                res.push_str(&" ".repeat(avg_spaces));
                if extra_spaces > 0 {
                    res.push(' ');
                    extra_spaces -= 1;
                }
            }
        }
        res
    }

    fn compact_last(words: Vec<String>, max_width: usize) -> String {
        let mut res = String::new();
        for (i, word) in words.iter().enumerate() {
            res.push_str(word);
            if i < words.len() - 1 {
                res.push(' ');
            }
        }
        res.push_str(&" ".repeat(max_width - res.len()));
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_68() {
        assert_eq!(
            Solution::full_justify(
                vec_string![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            vec_string!["This    is    an", "example  of text", "justification.  "]
        );

        assert_eq!(
            Solution::full_justify(
                vec_string!["What", "must", "be", "acknowledgment", "shall", "be"],
                16
            ),
            vec_string!["What   must   be", "acknowledgment  ", "shall be        "]
        );

        assert_eq!(
            Solution::full_justify(
                vec_string![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20
            ),
            vec_string![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  ",
            ]
        );
    }
}

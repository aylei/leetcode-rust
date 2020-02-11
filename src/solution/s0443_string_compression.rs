/**
 * [443] String Compression
 *
 * Given an array of characters, compress it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>.
 *
 * The length after compression must always be smaller than or equal to the original array.
 *
 * Every element of the array should be a character (not int) of length 1.
 *
 * After you are done modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>, return the new length of the array.
 *
 *
 * Follow up:<br />
 * Could you solve it using only O(1) extra space?
 *
 *
 * Example 1:
 *
 *
 * Input:
 * ["a","a","b","b","c","c","c"]
 *
 * Output:
 * Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
 *
 * Explanation:
 * "aa" is replaced by "a2". "bb" is replaced by "b2". "ccc" is replaced by "c3".
 *
 *
 *
 *
 * Example 2:
 *
 *
 * Input:
 * ["a"]
 *
 * Output:
 * Return 1, and the first 1 characters of the input array should be: ["a"]
 *
 * Explanation:
 * Nothing is replaced.
 *
 *
 *
 *
 * Example 3:
 *
 *
 * Input:
 * ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
 *
 * Output:
 * Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
 *
 * Explanation:
 * Since the character "a" does not repeat, it is not compressed. "bbbbbbbbbbbb" is replaced by "b12".
 * Notice each digit has it's own entry in the array.
 *
 *
 *
 *
 * Note:
 *
 * <ol>
 * 	All characters have an ASCII value in [35, 126].
 * 	1 <= len(chars) <= 1000.
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        use std::char;
        // add sentinel
        chars.push('0');
        let mut left = 0;
        let mut cnt: u32 = 1;
        for i in 1..chars.len() {
            let curr = chars[i];
            if chars[left] == curr {
                cnt += 1;
            } else {
                if cnt > 1 {
                    for c in cnt.to_string().chars() {
                        left += 1;
                        chars[left] = c;
                    }
                }
                left += 1;
                chars[left] = chars[i];
                cnt = 1;
            }
        }
        return left as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_443() {}
}

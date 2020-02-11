/**
 * [344] Reverse String
 *
 * Write a function that reverses a string. The input string is given as an array of characters char[].
 *
 * Do not allocate extra space for another array, you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 *
 * You may assume all the characters consist of <a href="https://en.wikipedia.org/wiki/ASCII#Printable_characters" target="_blank">printable ascii characters</a>.
 *
 *
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">["h","e","l","l","o"]</span>
 * Output: <span id="example-output-1">["o","l","l","e","h"]</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">["H","a","n","n","a","h"]</span>
 * Output: <span id="example-output-2">["h","a","n","n","a","H"]</span>
 *
 * </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            let tmp = s[left];
            s[left] = s[right];
            s[right] = tmp;
            left += 1;
            right -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_344() {
        let mut s = vec![];
        Solution::reverse_string(&mut s);
        println!("{:?}", s);
    }
}

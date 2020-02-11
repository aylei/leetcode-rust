/**
 * [434] Number of Segments in a String
 *
 * Count the number of segments in a string, where a segment is defined to be a contiguous sequence of non-space characters.
 *
 * Please note that the string does not contain any non-printable characters.
 *
 * Example:
 *
 * Input: "Hello, my name is John"
 * Output: 5
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut ret = 0;
        let mut prev_is_space = true;
        for c in s.chars() {
            let curr_is_space = c.is_whitespace();
            if prev_is_space && !curr_is_space {
                ret += 1;
            }
            prev_is_space = curr_is_space;
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_434() {}
}

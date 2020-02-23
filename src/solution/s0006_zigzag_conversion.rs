/**
 * [6] ZigZag Conversion
 *
 * The string <code>"PAYPALISHIRING"</code> is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: <code>"PAHNAPLSIIGYIR"</code>
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let unit: usize = (2 * num_rows - 2) as usize;
        let mut sb = String::with_capacity(s.capacity());
        for row in 0..num_rows {
            let mut i = row as usize;
            let step1 = 2 * (num_rows - 1 - row) as usize;
            let step2 = (unit - step1) as usize;
            let mut trigger = false;
            while i < len {
                sb.push(seq[i]);
                if step1 == 0 {
                    i += step2
                } else if step2 == 0 {
                    i += step1
                } else {
                    i += if trigger { step2 } else { step1 };
                    trigger = !trigger;
                }
            }
        }
        sb
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}

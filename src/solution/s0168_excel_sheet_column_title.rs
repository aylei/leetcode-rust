/**
 * [168] Excel Sheet Column Title
 *
 * Given a positive integer, return its corresponding column title as appear in an Excel sheet.
 *
 * For example:
 *
 *
 *     1 -> A
 *     2 -> B
 *     3 -> C
 *     ...
 *     26 -> Z
 *     27 -> AA
 *     28 -> AB
 *     ...
 *
 *
 * Example 1:
 *
 *
 * Input: 1
 * Output: "A"
 *
 *
 * Example 2:
 *
 *
 * Input: 28
 * Output: "AB"
 *
 *
 * Example 3:
 *
 *
 * Input: 701
 * Output: "ZY"
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/excel-sheet-column-title/
// discuss: https://leetcode.com/problems/excel-sheet-column-title/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let base = 26;
        let mut n = n;
        let mut res = Vec::new();
        while n > 0 {
            let mut code = (n % base) as u8;
            n = n / base;
            if code == 0 {
                n -= 1;
                code = base as u8;
            };
            let alphabetic = (('A' as u8) + (code - 1_u8)) as char;
            res.push(alphabetic);
        }
        res.reverse();
        res.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168() {
        assert_eq!(Solution::convert_to_title(28), "AB".to_owned());
        assert_eq!(Solution::convert_to_title(1), "A".to_owned());
    }
}

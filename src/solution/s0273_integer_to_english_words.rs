/**
 * [273] Integer to English Words
 *
 * Convert a non-negative integer to its english words representation. Given input is guaranteed to be less than 2^31 - 1.
 *
 * Example 1:
 *
 *
 * Input: 123
 * Output: "One Hundred Twenty Three"
 *
 *
 * Example 2:
 *
 *
 * Input: 12345
 * Output: "Twelve Thousand Three Hundred Forty Five"
 *
 * Example 3:
 *
 *
 * Input: 1234567
 * Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
 *
 *
 * Example 4:
 *
 *
 * Input: 1234567891
 * Output: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-to-english-words/
// discuss: https://leetcode.com/problems/integer-to-english-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// static digits: Vec<&str> = vec!["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
// static xteens: Vec<&str> = vec!["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventenn", "Eighteen", "Nineteen"];
// static xtys: Vec<&str> = vec!["Twenty", "Thirty", "Fourty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
// static hunderd: &str = "Hunderd";
// static thousands: Vec<&str> = vec!["Thousand", "Million", "Billion"];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        "".to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_273() {}
}

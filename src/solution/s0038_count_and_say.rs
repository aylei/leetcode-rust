/**
 * [38] Count and Say
 *
 * The count-and-say sequence is the sequence of integers with the first five terms as following:
 *
 *
 * 1.     1
 * 2.     11
 * 3.     21
 * 4.     1211
 * 5.     111221
 *
 *
 * 1 is read off as "one 1" or 11.<br />
 * 11 is read off as "two 1s" or 21.<br />
 * 21 is read off as "one 2, then one 1" or 1211.
 *
 * Given an integer n where 1 <= n <= 30, generate the n^th term of the count-and-say sequence.
 *
 * Note: Each term of the sequence of integers will be represented as a string.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: 1
 * Output: "1"
 *
 *
 * Example 2:
 *
 *
 * Input: 4
 * Output: "1211"
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-and-say/
// discuss: https://leetcode.com/problems/count-and-say/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::char::from_digit;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res = vec!['1'];
        for _ in 0..n - 1 {
            let mut temp = Vec::new();
            let mut i = 0_usize;
            while i < res.len() {
                let mut j = i + 1;
                while j < res.len() && res[j] == res[i] {
                    j += 1;
                }
                temp.push(from_digit((j - i) as u32, 10).unwrap());
                temp.push(res[i]);
                i = j;
            }
            res = temp;
        }
        res.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}

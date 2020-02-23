/**
 * [306] Additive Number
 *
 * Additive number is a string whose digits can form additive sequence.
 *
 * A valid additive sequence should contain at least three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.
 *
 * Given a string containing only digits '0'-'9', write a function to determine if it's an additive number.
 *
 * Note: Numbers in the additive sequence cannot have leading zeros, so sequence 1, 2, 03 or 1, 02, 3 is invalid.
 *
 * Example 1:
 *
 *
 * Input: "112358"
 * Output: true
 * Explanation: The digits can form an additive sequence: 1, 1, 2, 3, 5, 8.
 *              1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
 *
 *
 * Example 2:
 *
 *
 * Input: "199100199"
 * Output: true
 * Explanation: The additive sequence is: 1, 99, 100, 199<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">.</span>
 *              1 + 99 = 100, 99 + 100 = 199
 *
 * Follow up:<br />
 * How would you handle overflow for very large input integers?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/additive-number/
// discuss: https://leetcode.com/problems/additive-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// first_cut second_cut  third_cut
//     V         V          V
// 1       99        100          199
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let mut chs: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut num1 = 0;
        let len = chs.len();
        // first cut
        for i in 0..(len / 2 + 1) {
            num1 = num1 * 10 + chs[i];
            if Solution::second_cut(i + 1, len, num1, &chs) {
                return true;
            }
            if num1 == 0 {
                break;
            }
        }
        false
    }

    fn second_cut(from: usize, len: usize, num1: u32, chs: &Vec<u32>) -> bool {
        let mut num2 = 0;
        for i in from..len {
            num2 = num2 * 10 + chs[i];
            if Solution::third_cut(i + 1, len, num1, num2, chs, false) {
                return true;
            }
            if num2 == 0 {
                break;
            }
        }
        false
    }

    fn third_cut(
        from: usize,
        len: usize,
        num1: u32,
        num2: u32,
        chs: &Vec<u32>,
        found: bool,
    ) -> bool {
        if found && from >= len {
            return true;
        }
        let mut num3 = 0;
        for i in from..len {
            num3 = num3 * 10 + chs[i];
            if num3 == num2 + num1 {
                if Solution::third_cut(i + 1, len, num2, num3, chs, true) {
                    return true;
                }
            } else if num3 == 0 || num3 > num1 + num2 {
                break;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_306() {
        assert_eq!(Solution::is_additive_number("112358".to_owned()), true);
        assert_eq!(Solution::is_additive_number("199100199".to_owned()), true);
        assert_eq!(Solution::is_additive_number("1991001990".to_owned()), false);
        assert_eq!(Solution::is_additive_number("1023".to_owned()), false);
    }
}

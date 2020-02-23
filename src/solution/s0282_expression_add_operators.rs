/**
 * [282] Expression Add Operators
 *
 * Given a string that contains only digits 0-9 and a target value, return all possibilities to add binary operators (not unary) +, -, or * between the digits so they evaluate to the target value.
 *
 * Example 1:
 *
 *
 * Input: num = "123", target = 6
 * Output: ["1+2+3", "1*2*3"]
 *
 *
 * Example 2:
 *
 *
 * Input: num = "232", target = 8
 * Output: ["2*3+2", "2+3*2"]
 *
 * Example 3:
 *
 *
 * Input: num = "105", target = 5
 * Output: ["1*0+5","10-5"]
 *
 * Example 4:
 *
 *
 * Input: num = "00", target = 0
 * Output: ["0+0", "0-0", "0*0"]
 *
 *
 * Example 5:
 *
 *
 * Input: num = "3456237490", target = 9191
 * Output: []
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/expression-add-operators/
// discuss: https://leetcode.com/problems/expression-add-operators/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_282() {}
}

/**
 * [65] Valid Number
 *
 * Validate if a given string can be interpreted as a decimal number.
 *
 * Some examples:<br />
 * "0" => true<br />
 * " 0.1 " => true<br />
 * "abc" => false<br />
 * "1 a" => false<br />
 * "2e10" => true<br />
 * " -90e3   " => true<br />
 * " 1e" => false<br />
 * "e3" => false<br />
 * " 6e-1" => true<br />
 * " 99e2.5 " => false<br />
 * "53.5e93" => true<br />
 * " --6 " => false<br />
 * "-+3" => false<br />
 * "95a54e53" => false
 *
 * Note: It is intended for the problem statement to be ambiguous. You should gather all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:
 *
 *
 * 	Numbers 0-9
 * 	Exponent - "e"
 * 	Positive/negative sign - "+"/"-"
 * 	Decimal point - "."
 *
 *
 * Of course, the context of these characters also matters in the input.
 *
 * Update (2015-02-10):<br />
 * The signature of the C++ function had been updated. If you still see your function signature accepts a const char * argument, please click the reload button to reset your code definition.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-number/
// discuss: https://leetcode.com/problems/valid-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// hope that regex was included in std library...
// TODO: NFA
impl Solution {
    pub fn is_number(s: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65() {}
}

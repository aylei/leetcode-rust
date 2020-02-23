/**
 * [241] Different Ways to Add Parentheses
 *
 * Given a string of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. The valid operators are +, - and *.
 *
 * Example 1:
 *
 *
 * Input: "2-1-1"
 * Output: [0, 2]
 * Explanation:
 * ((2-1)-1) = 0
 * (2-(1-1)) = 2
 *
 * Example 2:
 *
 *
 * Input: "2*3-4*5"
 * Output: [-34, -14, -10, -10, 10]
 * Explanation:
 * (2*(3-(4*5))) = -34
 * ((2*3)-(4*5)) = -14
 * ((2*(3-4))*5) = -10
 * (2*((3-4)*5)) = -10
 * (((2*3)-4)*5) = 10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/different-ways-to-add-parentheses/
// discuss: https://leetcode.com/problems/different-ways-to-add-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        Solution::helper(&input)
    }

    pub fn helper(input: &str) -> Vec<i32> {
        if input.is_empty() {
            return vec![];
        }
        if let Ok(digit) = input.parse::<i32>() {
            return vec![digit];
        }
        let mut res: Vec<i32> = Vec::new();
        for (i, ch) in input.chars().enumerate() {
            if ch == '+' || ch == '-' || ch == '*' {
                let left = Solution::helper(&input[..i]);
                let right = Solution::helper(&input[i + 1..]);
                for &a in left.iter() {
                    for &b in right.iter() {
                        res.push(match ch {
                            '+' => a + b,
                            '-' => a - b,
                            '*' => a * b,
                            _ => unreachable!(),
                        })
                    }
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_241() {
        assert_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_owned()),
            vec![-34, -10, -14, -10, 10]
        );
    }
}

/**
 * [301] Remove Invalid Parentheses
 *
 * Remove the minimum number of invalid parentheses in order to make the input string valid. Return all possible results.
 *
 * Note: The input string may contain letters other than the parentheses ( and ).
 *
 * Example 1:
 *
 *
 * Input: "()())()"
 * Output: ["()()()", "(())()"]
 *
 *
 * Example 2:
 *
 *
 * Input: "(a)())()"
 * Output: ["(a)()()", "(a())()"]
 *
 *
 * Example 3:
 *
 *
 * Input: ")("
 * Output: [""]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-invalid-parentheses/
// discuss: https://leetcode.com/problems/remove-invalid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// 1. Calculate the number of misplaced left parenthese and right parenthese
// 2. DFS the string to find the all possible removing policy
use std::collections::HashSet;
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let (mut left, mut right) = (0, 0);
        let mut chs: Vec<char> = s.chars().collect();
        for &c in chs.iter() {
            if c == '(' {
                left += 1;
            } else if c == ')' {
                if left > 0 {
                    left -= 1;
                } else {
                    right += 1;
                }
            }
        }

        // Now, the number of left and right parentheses are 'left' and 'right'
        let mut res: HashSet<String> = HashSet::new();
        let mut seed: Vec<char> = Vec::new();
        Solution::helper(&chs, 0, 0, left, right, &mut seed, &mut res);
        res.into_iter().collect()
    }

    fn helper(
        chs: &Vec<char>,
        idx: usize,
        left: i32,
        l_remain: i32,
        r_remain: i32,
        exp: &mut Vec<char>,
        res: &mut HashSet<String>,
    ) {
        if idx >= chs.len() {
            if left == 0 {
                res.insert(exp.iter().collect());
            }
            return;
        }
        if chs[idx] == '(' {
            if l_remain > 0 {
                Solution::helper(
                    chs,
                    idx + 1,
                    left,
                    l_remain - 1,
                    r_remain,
                    &mut exp.clone(),
                    res,
                );
            }
            exp.push('(');
            Solution::helper(chs, idx + 1, left + 1, l_remain, r_remain, exp, res);
        } else if chs[idx] == ')' {
            if r_remain > 0 {
                Solution::helper(
                    chs,
                    idx + 1,
                    left,
                    l_remain,
                    r_remain - 1,
                    &mut exp.clone(),
                    res,
                );
            }
            if left > 0 {
                exp.push(')');
                Solution::helper(chs, idx + 1, left - 1, l_remain, r_remain, exp, res);
            }
        } else {
            exp.push(chs[idx]);
            Solution::helper(chs, idx + 1, left, l_remain, r_remain, exp, res);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_301() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_owned()),
            vec_string!["(())()", "()()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_owned()),
            vec_string!["(a)()()", "(a())()"]
        );
    }
}

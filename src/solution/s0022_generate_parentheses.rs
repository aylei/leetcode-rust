/**
 * [22] Generate Parentheses
 *
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 *
 *
 * For example, given n = 3, a solution set is:
 *
 *
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// DFS
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return vec![];
        }
        let mut result = Vec::new();
        Solution::dfs(n, 0, 0, &mut result, String::new());
        result
    }

    fn dfs(n: i32, left: i32, right: i32, result: &mut Vec<String>, mut path: String) {
        if left == n && right == n {
            result.push(path);
            return;
        }
        if left < n {
            let mut new_path = path.clone();
            new_path.push('(');
            Solution::dfs(n, left + 1, right, result, new_path);
        }
        if right < left {
            // reuse path to avoid clone overhead
            path.push(')');
            Solution::dfs(n, left, right + 1, result, path);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}

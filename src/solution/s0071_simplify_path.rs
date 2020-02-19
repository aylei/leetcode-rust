/**
 * [71] Simplify Path
 *
 * Given an absolute path for a file (Unix-style), simplify it. Or in other words, convert it to the canonical path.
 *
 * In a UNIX-style file system, a period . refers to the current directory. Furthermore, a double period .. moves the directory up a level. For more information, see: <a href="https://www.linuxnix.com/abslute-path-vs-relative-path-in-linuxunix/" target="_blank">Absolute path vs relative path in Linux/Unix</a>
 *
 * Note that the returned canonical path must always begin with a slash /, and there must be only a single slash / between two directory names. The last directory name (if it exists) must not end with a trailing /. Also, the canonical path must be the shortest string representing the absolute path.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: "<span id="example-input-1-1">/home/"</span>
 * Output: "<span id="example-output-1">/home"
 * Explanation: Note that there is no trailing slash after the last directory name.</span>
 *
 *
 * Example 2:
 *
 *
 * Input: "<span id="example-input-1-1">/../"</span>
 * Output: "<span id="example-output-1">/"</span>
 * Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
 *
 *
 * Example 3:
 *
 *
 * Input: "<span id="example-input-1-1">/home//foo/"</span>
 * Output: "<span id="example-output-1">/home/foo"</span>
 * Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
 *
 *
 * Example 4:
 *
 *
 * Input: "<span id="example-input-1-1">/a/./b/../../c/"</span>
 * Output: "<span id="example-output-1">/c"</span>
 *
 *
 * Example 5:
 *
 *
 * Input: "<span id="example-input-1-1">/a/../../b/../c//.//"</span>
 * Output: "<span id="example-output-1">/c"</span>
 *
 *
 * Example 6:
 *
 *
 * Input: "<span id="example-input-1-1">/a//b////c/d//././/.."</span>
 * Output: "<span id="example-output-1">/a/b/c"</span>
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/simplify-path/
// discuss: https://leetcode.com/problems/simplify-path/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for s in path.split('/') {
            match s {
                "." => {}
                "/" => {}
                "" => {}
                ".." => {
                    stack.pop();
                }
                _ => stack.push(s),
            }
        }
        let mut res = String::new();
        for s in stack {
            res.push('/');
            res.push_str(s);
        }
        if res.len() > 0 {
            res
        } else {
            "/".to_owned()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_71() {
        assert_eq!(Solution::simplify_path("/home/".to_owned()), "/home");
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/");
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_owned()), "/c");
        assert_eq!(
            Solution::simplify_path("/a/../../b/../c//.//".to_owned()),
            "/c"
        );
        assert_eq!(
            Solution::simplify_path("/a//b////c/d//././/..".to_owned()),
            "/a/b/c"
        );
    }
}

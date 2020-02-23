/**
 * [165] Compare Version Numbers
 *
 * Compare two version numbers version1 and version2.<br />
 * If version1 > version2 return 1; if version1 < version2 return -1;otherwise return 0.
 *
 * You may assume that the version strings are non-empty and contain only digits and the . character.
 * The . character does not represent a decimal point and is used to separate number sequences.
 * For instance, 2.5 is not "two and a half" or "half way to version three", it is the fifth second-level revision of the second first-level revision.
 * You may assume the default revision number for each level of a version number to be 0. For example, version number 3.4 has a revision number of 3 and 4 for its first and second level revision number. Its third and fourth level revision number are both 0.
 *
 *  
 *
 * Example 1:
 *
 * Input: version1 = "0.1", version2 = "1.1"
 * Output: -1
 *
 * Example 2:
 *
 * Input: version1 = "1.0.1", version2 = "1"
 * Output: 1
 *
 * Example 3:
 *
 * Input: version1 = "7.5.2.4", version2 = "7.5.3"
 * Output: -1
 *
 * Example 4:
 *
 * Input: version1 = "1.01", version2 = "1.001"
 * Output: 0
 * Explanation: Ignoring leading zeroes, both “01” and “001" represent the same number “1”
 *
 * Example 5:
 *
 * Input: version1 = "1.0", version2 = "1.0.0"
 * Output: 0
 * Explanation: The first version number does not have a third level revision number, which means its third level revision number is default to "0"
 *
 *  
 *
 * Note:
 * <ol>
 * Version strings are composed of numeric strings separated by dots . and this numeric strings may have leading zeroes.
 * Version strings do not start or end with dots, and they will not be two consecutive dots.
 * </ol>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/compare-version-numbers/
// discuss: https://leetcode.com/problems/compare-version-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<&str> = version1.split('.').collect::<Vec<_>>();
        let v2: Vec<&str> = version2.split('.').collect::<Vec<_>>();
        let mut i = 0_usize;
        while i < v1.len() && i < v2.len() {
            let left = v1[i].parse::<u32>().unwrap();
            let right = v2[i].parse::<u32>().unwrap();
            if left > right {
                return 1;
            } else if left < right {
                return -1;
            }
            i += 1;
        }
        while i < v1.len() {
            if v1[i].parse::<u32>().unwrap() > 0 {
                return 1;
            }
            i += 1;
        }
        while i < v2.len() {
            if v2[i].parse::<u32>().unwrap() > 0 {
                return -1;
            }
            i += 1;
        }
        return 0;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_165() {
        assert_eq!(
            Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
            1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.01".to_owned(), "1.0001".to_owned()),
            0
        );
    }
}

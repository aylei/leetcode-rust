/**
 * [274] H-Index
 *
 * Given an array of citations (each citation is a non-negative integer) of a researcher, write a function to compute the researcher's h-index.
 *
 * According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: "A scientist has index h if h of his/her N papers have at least h citations each, and the other N - h papers have no more than h citations each."
 *
 * Example:
 *
 *
 * Input: citations = [3,0,6,1,5]
 * Output: 3
 * Explanation: [3,0,6,1,5] means the researcher has 5 papers in total and each of them had
 *              received 3, 0, 6, 1, 5 citations respectively.
 *              Since the researcher has 3 papers with at least 3 citations each and the remaining
 *              two with no more than 3 citations each, her h-index is 3.
 *
 * Note: If there are several possible values for h, the maximum one is taken as the h-index.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/h-index/
// discuss: https://leetcode.com/problems/h-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let mut hidx = 0;
        for &v in citations.iter().rev() {
            if v >= hidx + 1 {
                hidx += 1;
            } else {
                break;
            }
        }
        hidx
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_274() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}

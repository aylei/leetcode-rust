/**
 * [275] H-Index II
 *
 * Given an array of citations sorted in ascending order (each citation is a non-negative integer) of a researcher, write a function to compute the researcher's h-index.
 *
 * According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: "A scientist has index h if h of his/her N papers have at least h citations each, and the other N - h papers have no more than h citations each."
 *
 * Example:
 *
 *
 * Input: citations = [0,1,3,5,6]
 * Output: 3
 * Explanation: [0,1,3,5,6] means the researcher has 5 papers in total and each of them had
 *              received 0, 1, 3, 5, 6 citations respectively.
 *              Since the researcher has 3 papers with at least 3 citations each and the remaining
 *              two with no more than 3 citations each, her h-index is 3.
 *
 * Note:
 *
 * If there are several possible values for h, the maximum one is taken as the h-index.
 *
 * Follow up:
 *
 *
 * 	This is a follow up problem to <a href="/problems/h-index/description/">H-Index</a>, where citations is now guaranteed to be sorted in ascending order.
 * 	Could you solve it in logarithmic time complexity?
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/h-index-ii/
// discuss: https://leetcode.com/problems/h-index-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        if citations.is_empty() {
            return 0;
        }
        let len = citations.len();
        let (mut low, mut high) = (0_usize, len - 1);
        while low <= high {
            let mid = low + (high - low) / 2;
            if citations[mid] >= (len - mid) as i32 {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        (len - low) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_275() {
        assert_eq!(Solution::h_index(vec![]), 0);
        assert_eq!(Solution::h_index(vec![0]), 0);
        assert_eq!(Solution::h_index(vec![11, 15]), 2);
        assert_eq!(Solution::h_index(vec![1]), 1);
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(Solution::h_index(vec![0, 4, 4, 5, 6]), 4);
    }
}

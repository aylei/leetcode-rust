/**
 * [75] Sort Colors
 *
 * Given an array with n objects colored red, white or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white and blue.
 *
 * Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
 *
 * Note: You are not suppose to use the library's sort function for this problem.
 *
 * Example:
 *
 *
 * Input: [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 *
 * Follow up:
 *
 *
 * 	A rather straight forward solution is a two-pass algorithm using counting sort.<br />
 * 	First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
 * 	Could you come up with a one-pass algorithm using only constant space?
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// three-way partition
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }
        let (mut lower_idx, mut upper_idx) = (0_usize, nums.len() - 1);
        let mut i = 0_usize;
        while i <= upper_idx {
            if nums[i] < 1 {
                // lower_idx <= i, we've scanned it so we know nums[lower_idx] <= 1, i++
                nums.swap(lower_idx, i);
                i += 1;
                lower_idx += 1;
            } else if nums[i] > 1 {
                nums.swap(upper_idx, i);
                if upper_idx < 1 {
                    break;
                }
                upper_idx -= 1;
            } else {
                i += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut vec = vec![
            1, 2, 0, 1, 2, 2, 2, 0, 0, 0, 2, 1, 1, 2, 0, 1, 2, 2, 1, 1, 0,
        ];
        Solution::sort_colors(&mut vec);
        assert_eq!(
            vec,
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2]
        );

        let mut vec = vec![];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![]);

        let mut vec = vec![2, 2, 2];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![2, 2, 2]);
    }
}

/**
 * [11] Container With Most Water
 *
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.
 *
 * Note: You may not slant the container and n is at least 2.
 *
 *  
 *
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;" />
 *
 * <small>The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49. </small>
 *
 *  
 *
 * Example:
 *
 *
 * Input: [1,8,6,2,5,4,8,3,7]
 * Output: 49
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/container-with-most-water/
// discuss: https://leetcode.com/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Brute force: O(N^2)

// Two Pointer: a[0] ->    <- a[n-1]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0_usize, (height.len() - 1));
        let mut max: i32 = (end - start) as i32 * Solution::min(height[start], height[end]);
        let mut curr_area: i32 = 0;
        while end - start > 1 {
            // move the lower one
            if height[start] < height[end] {
                start += 1;
                if height[start] < height[start - 1] {
                    continue;
                }
            } else {
                end -= 1;
                if height[end] < height[end + 1] {
                    continue;
                }
            }
            curr_area = (end - start) as i32 * Solution::min(height[start], height[end]);
            if curr_area > max {
                max = curr_area
            }
        }
        max
    }

    #[inline(always)]
    fn min(i: i32, j: i32) -> i32 {
        if i > j {
            j
        } else {
            i
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![6, 9]), 6);
        assert_eq!(Solution::max_area(vec![1, 1, 2, 1, 1, 1]), 5);
    }
}

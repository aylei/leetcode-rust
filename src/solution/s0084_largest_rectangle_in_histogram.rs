/**
 * [84] Largest Rectangle in Histogram
 *
 * Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.
 *
 *  
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/histogram.png" style="width: 188px; height: 204px;" /><br />
 * <small>Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].</small>
 *
 *  
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/histogram_area.png" style="width: 188px; height: 204px;" /><br />
 * <small>The largest rectangle is shown in the shaded area, which has area = 10 unit.</small>
 *
 *  
 *
 * Example:
 *
 *
 * Input: [2,1,5,6,2,3]
 * Output: 10
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-rectangle-in-histogram/
// discuss: https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// record the height and start position using 2 stack, thus we reuse the previously scanned information
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut positions = Vec::new();
        let mut hs = Vec::new();
        let mut max_area = 0;
        let len = heights.len();
        for (i, h) in heights.into_iter().enumerate() {
            let mut last_pop = None;
            while hs.last().is_some() && *hs.last().unwrap() >= h {
                max_area = i32::max(
                    max_area,
                    hs.last().unwrap() * ((i - positions.last().unwrap()) as i32),
                );
                hs.pop();
                last_pop = positions.pop();
            }
            if last_pop.is_some() {
                positions.push(last_pop.unwrap());
            } else {
                positions.push(i);
            }
            hs.push(h);
        }
        // drain stack
        while !hs.is_empty() {
            max_area = i32::max(
                max_area,
                hs.last().unwrap() * ((len - positions.last().unwrap()) as i32),
            );
            positions.pop();
            hs.pop();
        }
        max_area
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(
            Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1]),
            8
        );
        assert_eq!(Solution::largest_rectangle_area(vec![2, 2]), 4);
        assert_eq!(
            Solution::largest_rectangle_area(vec![1, 2, 8, 8, 2, 2, 1]),
            16
        );
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
        assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 2]), 5);
    }
}

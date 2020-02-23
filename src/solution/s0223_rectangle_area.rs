/**
 * [223] Rectangle Area
 *
 * Find the total area covered by two rectilinear rectangles in a 2D plane.
 *
 * Each rectangle is defined by its bottom left corner and top right corner as shown in the figure.
 *
 * <img alt="Rectangle Area" src="https://assets.leetcode.com/uploads/2018/10/22/rectangle_area.png" style="width: 542px; height: 304px;" />
 *
 * Example:
 *
 *
 * Input: A = <span id="example-input-1-1">-3</span>, B = <span id="example-input-1-2">0</span>, C = <span id="example-input-1-3">3</span>, D = <span id="example-input-1-4">4</span>, E = <span id="example-input-1-5">0</span>, F = <span id="example-input-1-6">-1</span>, G = <span id="example-input-1-7">9</span>, H = <span id="example-input-1-8">2</span>
 * Output: <span id="example-output-1">45</span>
 *
 * Note:
 *
 * Assume that the total area is never beyond the maximum possible value of int.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rectangle-area/
// discuss: https://leetcode.com/problems/rectangle-area/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// mention the integer divition
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let center1 = ((a + c), (b + d));
        let center2 = ((e + g), (f + h));
        let rect1 = (c - a, d - b);
        let rect2 = (g - e, h - f);
        let x_intersect = i32::min(
            (rect1.0 + rect2.0 - (center1.0 - center2.0).abs()),
            i32::min(2 * rect1.0, 2 * rect2.0),
        ) / 2;
        let x_intersect = if x_intersect < 0 { 0 } else { x_intersect };
        let y_intersect = i32::min(
            (rect1.1 + rect2.1 - (center1.1 - center2.1).abs()),
            i32::min(2 * rect1.1, 2 * rect2.1),
        ) / 2;
        let y_intersect = if y_intersect < 0 { 0 } else { y_intersect };
        rect1.0 * rect1.1 - x_intersect * y_intersect + rect2.0 * rect2.1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_223() {
        assert_eq!(Solution::compute_area(0, 0, 0, 0, 0, 0, 0, 0), 0);
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -1, 4, 1, 6), 20);
    }
}

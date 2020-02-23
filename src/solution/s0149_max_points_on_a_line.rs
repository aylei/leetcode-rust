/**
 * [149] Max Points on a Line
 *
 * Given n points on a 2D plane, find the maximum number of points that lie on the same straight line.
 *
 * Example 1:
 *
 *
 * Input: [[1,1],[2,2],[3,3]]
 * Output: 3
 * Explanation:
 * ^
 * |
 * |        o
 * |     o
 * |  o
 * +------------->
 * 0  1  2  3  4
 *
 *
 * Example 2:
 *
 *
 * Input: [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 * Explanation:
 * ^
 * |
 * |  o
 * |     o        o
 * |        o
 * |  o        o
 * +------------------->
 * 0  1  2  3  4  5  6
 *
 *
 */
pub struct Solution {}
use crate::util::point::Point;

// problem: https://leetcode.com/problems/max-points-on-a-line/
// discuss: https://leetcode.com/problems/max-points-on-a-line/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
要回顾下高中数学：已知两点, 求解一般式:

  * Ax + By + C = 0
  * A = y2 - y1, B = x1 - x2, C = x2y1 - x1y2

有这个知识之后，化为一般式，做三层遍历就行，再加上一个 HashSet，避免对同一直线上点的重复计算，时间复杂度可以是 O(N^2)

有两个坑要注意避免：

  * 给的 case 会导致 i32 溢出，这里直接用了 i64 表示
  * 给的 case 里有相同的点，直接处理相同点的话会导致最坏情况复杂度到 O(N^3)，因此要先做一次转化，归并相同的点

用 Rust 实现有另一点注意的，给的 Point 没有实现 Hash Trait，要自己转化一下
*/
// straight-line expression: Ax + By + C = 0
// A = y2 - y1, B = x1 - x2, C = x2y1 - x1y2
#[derive(PartialEq, Hash, Eq, Debug)]
struct Line(i64, i64, i64);

impl Line {
    // Assumes that there is no same point
    fn new(p1: &Point, p2: &Point) -> Self {
        let x1 = p1.x as i64;
        let x2 = p2.x as i64;
        let y1 = p1.y as i64;
        let y2 = p2.y as i64;
        Line(y2 - y1, x1 - x2, x2 * y1 - x1 * y2)
    }
    fn contains(&self, p: &Point) -> bool {
        self.0 * p.x as i64 + self.1 * p.y as i64 + self.2 == 0_i64
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn max_points(points: Vec<Point>) -> i32 {
        // fold same point, record the point count
        let points: Vec<(Point, i32)> = points
            .into_iter()
            .fold(HashMap::new(), |mut map, v| {
                *map.entry((v.x, v.y)).or_insert(0) += 1;
                map
            })
            .into_iter()
            .map(|(k, v)| (Point::new(k.0, k.1), v)) // Point did not implement Hash trait
            .collect();

        // any two points in a straight-line, return quickly
        if points.len() < 3 {
            return points.into_iter().fold(0, |acc, v| acc + v.1);
        }
        let mut max = 2;
        let mut set: HashSet<Line> = HashSet::new();
        for i in 0..(points.len() - 1) {
            for j in i + 1..points.len() {
                let line = Line::new(&points[i].0, &points[j].0);
                if set.contains(&line) {
                    continue;
                }
                let mut curr = points[i].1 + points[j].1;
                for k in j + 1..points.len() {
                    if line.contains(&points[k].0) {
                        curr += points[k].1;
                    }
                }
                max = i32::max(max, curr);
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {
        assert_eq!(
            Solution::max_points(vec![point![1, 1], point![2, 2], point![3, 3]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                point![1, 1],
                point![3, 2],
                point![5, 3],
                point![4, 1],
                point![2, 3],
                point![1, 4]
            ]),
            4
        );
        assert_eq!(
            Solution::max_points(vec![point![0, 0], point![1, 65536], point![65536, 0]]),
            2
        );
        assert_eq!(
            Solution::max_points(vec![point![1, 1], point![1, 1], point![1, 1]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                point![0, 9],
                point![138, 429],
                point![115, 359],
                point![115, 359],
                point![-30, -102],
                point![230, 709],
                point![-150, -686],
                point![-135, -613],
                point![-60, -248],
                point![-161, -481],
                point![207, 639],
                point![23, 79],
                point![-230, -691],
                point![-115, -341],
                point![92, 289],
                point![60, 336],
                point![-105, -467],
                point![135, 701],
                point![-90, -394],
                point![-184, -551],
                point![150, 774]
            ]),
            12
        )
    }
}

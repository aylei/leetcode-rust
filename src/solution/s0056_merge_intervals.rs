/**
 * [56] Merge Intervals
 *
 * Given a collection of intervals, merge all overlapping intervals.
 *
 * Example 1:
 *
 *
 * Input: [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 *
 *
 * Example 2:
 *
 *
 * Input: [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}
impl Solution {
    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|interval| interval.start);
        let mut result: Vec<Interval> = Vec::new();
        for interval in intervals.into_iter() {
            match result.last_mut() {
                Some(mut last_inter) => {
                    if last_inter.end >= interval.start {
                        last_inter.end = i32::max(last_inter.end, interval.end);
                        continue;
                    }
                }
                None => {}
            }
            result.push(interval);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56() {
        assert_eq!(
            Solution::merge(vec![
                Interval::new(1, 3),
                Interval::new(2, 6),
                Interval::new(8, 10),
                Interval::new(15, 18)
            ]),
            vec![
                Interval::new(1, 6),
                Interval::new(8, 10),
                Interval::new(15, 18)
            ]
        );
    }
}

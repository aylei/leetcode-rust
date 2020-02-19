/**
 * [57] Insert Interval
 *
 * Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
 *
 * You may assume that the intervals were initially sorted according to their start times.
 *
 * Example 1:
 *
 *
 * Input: intervals = [[1,3],Interval::new(6,9)], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 *
 *
 * Example 2:
 *
 *
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-interval/
// discuss: https://leetcode.com/problems/insert-interval/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn insert(intervals: Vec<Interval>, new_interval: Interval) -> Vec<Interval> {
        let mut result = Vec::new();
        let mut new_interval = new_interval;
        let mut inserting = false;
        let mut inserted = false;
        for interval in intervals.into_iter() {
            if new_interval.start <= interval.end && !inserted {
                inserting = true;
            }
            if inserting {
                if new_interval.end >= interval.start {
                    new_interval.start = i32::min(new_interval.start, interval.start);
                    new_interval.end = i32::max(new_interval.end, interval.end);
                } else {
                    result.push(Interval::new(new_interval.start, new_interval.end));
                    inserting = false;
                    inserted = true;
                }
            }
            if !inserting {
                result.push(interval);
            }
        }
        if !inserted {
            result.push(new_interval);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_57() {
        assert_eq!(
            Solution::insert(
                vec![Interval::new(1, 3), Interval::new(6, 9)],
                Interval::new(2, 5)
            ),
            vec![Interval::new(1, 5), Interval::new(6, 9)]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    Interval::new(1, 2),
                    Interval::new(3, 5),
                    Interval::new(6, 7),
                    Interval::new(8, 10),
                    Interval::new(12, 16)
                ],
                Interval::new(4, 8)
            ),
            vec![
                Interval::new(1, 2),
                Interval::new(3, 10),
                Interval::new(12, 16)
            ]
        );
        assert_eq!(
            Solution::insert(vec![Interval::new(3, 4)], Interval::new(1, 2)),
            vec![Interval::new(1, 2), Interval::new(3, 4)]
        );
        assert_eq!(
            Solution::insert(vec![Interval::new(1, 2)], Interval::new(3, 4)),
            vec![Interval::new(1, 2), Interval::new(3, 4)]
        );
        assert_eq!(
            Solution::insert(vec![Interval::new(1, 2)], Interval::new(2, 3)),
            vec![Interval::new(1, 3)]
        );
        assert_eq!(
            Solution::insert(
                vec![Interval::new(1, 2), Interval::new(3, 4)],
                Interval::new(0, 6)
            ),
            vec![Interval::new(0, 6)]
        );
    }
}

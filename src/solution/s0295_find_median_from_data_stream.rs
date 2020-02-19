/**
 * [295] Find Median from Data Stream
 *
 * Median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value. So the median is the mean of the two middle value.
 * For example,
 *
 * [2,3,4], the median is 3
 *
 * [2,3], the median is (2 + 3) / 2 = 2.5
 *
 * Design a data structure that supports the following two operations:
 *
 *
 * 	void addNum(int num) - Add a integer number from the data stream to the data structure.
 * 	double findMedian() - Return the median of all elements so far.
 *
 *
 *
 *
 * Example:
 *
 *
 * addNum(1)
 * addNum(2)
 * findMedian() -> 1.5
 * addNum(3)
 * findMedian() -> 2
 *
 *
 *
 *
 * Follow up:
 *
 * <ol>
 * 	If all integer numbers from the stream are between 0 and 100, how would you optimize it?
 * 	If 99% of all integer numbers from the stream are between 0 and 100, how would you optimize it?
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-median-from-data-stream/
// discuss: https://leetcode.com/problems/find-median-from-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Eq, PartialEq)]
struct Invert {
    value: i32,
}

impl Ord for Invert {
    fn cmp(&self, other: &Invert) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Invert {
    fn partial_cmp(&self, other: &Invert) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MedianFinder {
    head: BinaryHeap<Invert>,
    tail: BinaryHeap<i32>,
    count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            head: BinaryHeap::new(),
            tail: BinaryHeap::new(),
            count: 0,
        }
    }

    fn add_num(&mut self, num: i32) {
        self.count += 1;
        if self.head.is_empty() || num > self.head.peek().unwrap().value {
            self.head.push(Invert { value: num });
        } else {
            self.tail.push(num);
        }
        // balance
        if self.head.len() > self.tail.len() + 1 {
            self.tail.push(self.head.pop().unwrap().value);
        } else if self.head.len() + 1 < self.tail.len() {
            self.head.push(Invert {
                value: self.tail.pop().unwrap(),
            });
        }
    }

    fn find_median(&self) -> f64 {
        if self.head.len() > self.tail.len() {
            self.head.peek().unwrap().value as f64
        } else if self.head.len() < self.tail.len() {
            *self.tail.peek().unwrap() as f64
        } else {
            (self.head.peek().unwrap().value as f64 + *self.tail.peek().unwrap() as f64) / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_295() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        assert_eq!(obj.find_median(), 2_f64);
    }
}

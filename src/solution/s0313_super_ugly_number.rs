/**
 * [313] Super Ugly Number
 *
 * Write a program to find the n^th super ugly number.
 *
 * Super ugly numbers are positive numbers whose all prime factors are in the given prime list primes of size k.
 *
 * Example:
 *
 *
 * Input: n = 12, primes = [2,7,13,19]
 * Output: 32
 * Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12
 *              super ugly numbers given primes = [2,7,13,19] of size 4.
 *
 * Note:
 *
 *
 * 	1 is a super ugly number for any given primes.
 * 	The given numbers in primes are in ascending order.
 * 	0 < k &le; 100, 0 < n &le; 10^6, 0 < primes[i] < 1000.
 * 	The n^th super ugly number is guaranteed to fit in a 32-bit signed integer.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-ugly-number/
// discuss: https://leetcode.com/problems/super-ugly-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Eq, PartialEq)]
struct Invert {
    base: i32,
    idx: usize,
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

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut vec = vec![1; 1];
        let mut heap: BinaryHeap<Invert> = BinaryHeap::new();
        for &prime in primes.iter() {
            heap.push(Invert {
                base: prime,
                idx: 0,
                value: prime,
            });
        }
        for _ in 0..n - 1 {
            let mut min = 0;
            if let Some(num) = heap.peek() {
                min = num.value;
            }
            vec.push(min);
            while heap.peek().unwrap().value == min {
                let p = heap.pop().unwrap();
                heap.push(Invert {
                    base: p.base,
                    idx: p.idx + 1,
                    value: p.base * vec[p.idx + 1],
                });
            }
        }
        *vec.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_313() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    }
}

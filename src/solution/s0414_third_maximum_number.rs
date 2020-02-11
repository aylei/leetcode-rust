/**
 * [414] Third Maximum Number
 *
 * Given a non-empty array of integers, return the third maximum number in this array. If it does not exist, return the maximum number. The time complexity must be in O(n).
 *
 * Example 1:<br />
 *
 * Input: [3, 2, 1]
 *
 * Output: 1
 *
 * Explanation: The third maximum is 1.
 *
 *
 *
 * Example 2:<br />
 *
 * Input: [1, 2]
 *
 * Output: 2
 *
 * Explanation: The third maximum does not exist, so the maximum (2) is returned instead.
 *
 *
 *
 * Example 3:<br />
 *
 * Input: [2, 2, 3, 1]
 *
 * Output: 1
 *
 * Explanation: Note that the third maximum here means the third maximum distinct number.
 * Both numbers with value 2 are both considered as second maximum.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        // use min heap
        let mut heap = vec![std::i64::MIN; 3];
        for num in nums {
            let num = num as i64;
            if heap.contains(&num) {
                continue;
            }
            if num > heap[0] {
                heap[0] = num;
                if heap[0] > heap[1] {
                    let tmp = heap[0];
                    heap[0] = heap[1];
                    heap[1] = tmp;
                }
                if heap[0] > heap[2] {
                    let tmp = heap[0];
                    heap[0] = heap[2];
                    heap[2] = tmp;
                }
            }
        }
        if heap.contains(&std::i64::MIN) {
            return heap[1].max(heap[2]) as i32;
        }
        heap[0] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_414() {
        println!("{}", Solution::third_max(vec![1, 2, 2, 4]))
    }
}

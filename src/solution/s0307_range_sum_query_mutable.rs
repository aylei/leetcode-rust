/**
 * [307] Range Sum Query - Mutable
 *
 * Given an integer array nums, find the sum of the elements between indices i and j (i &le; j), inclusive.
 *
 * The update(i, val) function modifies nums by updating the element at index i to val.
 *
 * Example:
 *
 *
 * Given nums = [1, 3, 5]
 *
 * sumRange(0, 2) -> 9
 * update(1, 2)
 * sumRange(0, 2) -> 8
 *
 *
 * Note:
 *
 * <ol>
 * 	The array is only modifiable by the update function.
 * 	You may assume the number of calls to update and sumRange function is distributed evenly.
 * </ol>
 *
 */
pub struct Solution {}

// Segement Tree
//
//                       N[0:6]
//                  /               \
//                 /                 \
//           N[0:3]                   N[4:6]
//         /        \               /        \
//     N[0:1]       N[2:3]      N[4:5]      N[6]
//    /    \        /    \     /     \
//   N[0]  N[1]   N[2]  N[3]  N[4]  N[5]
//
// problem: https://leetcode.com/problems/range-sum-query-mutable/
// discuss: https://leetcode.com/problems/range-sum-query-mutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumArray {
    tree: Vec<i32>,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; 4 * n];
        if n > 0 {
            NumArray::build(1, 0, n - 1, &mut tree, &nums);
        }
        NumArray { tree: tree, n: n }
    }

    fn update(&mut self, i: i32, val: i32) {
        NumArray::update_internal(i as usize, val, 1, 0, self.n - 1, &mut self.tree);
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        NumArray::sum(1, 0, self.n - 1, i as usize, j as usize, &self.tree)
    }

    fn build(node: usize, start: usize, end: usize, tree: &mut Vec<i32>, nums: &Vec<i32>) {
        if start == end {
            tree[node] = nums[start];
        } else {
            let mid = (start + end) / 2;
            NumArray::build(2 * node, start, mid, tree, nums);
            NumArray::build(2 * node + 1, mid + 1, end, tree, nums);
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }
    }

    fn update_internal(
        i: usize,
        val: i32,
        node: usize,
        start: usize,
        end: usize,
        tree: &mut Vec<i32>,
    ) {
        if start == end {
            tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            if i <= mid {
                NumArray::update_internal(i, val, 2 * node, start, mid, tree);
            } else {
                NumArray::update_internal(i, val, 2 * node + 1, mid + 1, end, tree);
            }
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }
    }

    fn sum(
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        tree: &Vec<i32>,
    ) -> i32 {
        if right < start || left > end {
            // not in range
            0
        } else if left <= start && end <= right {
            // completely in range
            tree[node]
        } else {
            // partially in range
            let mid = (start + end) / 2;
            NumArray::sum(2 * node, start, mid, left, right, tree)
                + NumArray::sum(2 * node + 1, mid + 1, end, left, right, tree)
        }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_307() {
        let _empty = NumArray::new(vec![]);
        let mut tree = NumArray::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(tree.sum_range(0, 6), 7);
        tree.update(0, 2);
        assert_eq!(tree.sum_range(0, 6), 8);
        tree.update(1, 2);
        assert_eq!(tree.sum_range(0, 2), 5);
        tree.update(6, 10);
        assert_eq!(tree.sum_range(6, 6), 10);
    }
}

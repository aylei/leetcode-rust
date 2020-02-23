/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 *
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 *
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and use only constant extra memory.
 *
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 &rarr; 1,3,2<br />
 * 3,2,1 &rarr; 1,2,3<br />
 * 1,1,5 &rarr; 1,5,1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut i = (len - 1) as i32;
        let mut prev = -1;
        // find the decrement digit from end
        while i >= 0 {
            if nums[i as usize] < prev {
                break;
            }
            prev = nums[i as usize];
            i -= 1;
        }
        let mut j = len - 1;
        // find the first digit larger than nums[i]
        // we can do binary search here to make a slightly improvement
        if i >= 0 {
            while j > (i as usize) {
                if nums[j] > nums[i as usize] {
                    nums.swap(i as usize, j);
                    break;
                }
                j -= 1;
            }
        }
        let slice = &mut nums[((i + 1) as usize)..len];
        slice.reverse();
    }
}

// submission codes end

/*
// a clean solution (from leetcode submissions)
impl Solution {
    pub fn next_permutation(a: &mut Vec<i32>) {
        let n = a.len();

        if let Some(i) = (1..n).rev().find(|&i| a[i - 1] < a[i]) {
            let j = (i..n).rev().find(|&j| a[i - 1] < a[j])
                .unwrap();

            a.swap(i - 1, j);
            a[i..].reverse();
        } else {
            a.reverse();
        }
    }
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);
    }
}

/**
 * [46] Permutations
 *
 * Given a collection of distinct integers, return all possible permutations.
 *
 * Example:
 *
 *
 * Input: [1,2,3]
 * Output:
 * [
 *   [1,2,3],
 *   [1,3,2],
 *   [2,1,3],
 *   [2,3,1],
 *   [3,1,2],
 *   [3,2,1]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations/
// discuss: https://leetcode.com/problems/permutations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }
        nums.iter()
            .flat_map(|&num| {
                let mut sub = nums.clone().into_iter().filter(|&x| x != num).collect();
                Solution::permute(sub)
                    .into_iter()
                    .map(|vec| {
                        let mut vec = vec;
                        vec.push(num);
                        vec
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![1, 2, 3],
            ]
        )
    }
}

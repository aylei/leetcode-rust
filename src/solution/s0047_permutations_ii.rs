/**
 * [47] Permutations II
 *
 * Given a collection of numbers that might contain duplicates, return all possible unique permutations.
 *
 * Example:
 *
 *
 * Input: [1,1,2]
 * Output:
 * [
 *   [1,1,2],
 *   [1,2,1],
 *   [2,1,1]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations-ii/
// discuss: https://leetcode.com/problems/permutations-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        Solution::permute(nums)
    }

    fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }
        let mut prev: Option<i32> = None;
        let mut res = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if prev.is_some() && prev.unwrap() == num {
                continue;
            } else {
                prev = Some(num)
            }
            let mut sub = nums.clone();
            sub.remove(i);
            let mut permutations: Vec<Vec<i32>> = Solution::permute(sub)
                .into_iter()
                .map(|x| {
                    let mut x = x;
                    x.push(num);
                    x
                })
                .collect();
            res.append(&mut permutations);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::permute(vec![1, 1, 2]),
            vec![vec![2, 1, 1], vec![1, 2, 1], vec![1, 1, 2],]
        );
        assert_eq!(Solution::permute(vec![1, 1, 1]), vec![vec![1, 1, 1],]);
        assert_eq!(
            Solution::permute(vec![1, 1, 1, 2]),
            vec![
                vec![2, 1, 1, 1],
                vec![1, 2, 1, 1],
                vec![1, 1, 2, 1],
                vec![1, 1, 1, 2],
            ]
        );
        assert_eq!(
            Solution::permute(vec![1, 1, 2, 2, 3, 3]),
            vec![
                vec![3, 3, 2, 2, 1, 1],
                vec![3, 2, 3, 2, 1, 1],
                vec![2, 3, 3, 2, 1, 1],
                vec![3, 2, 2, 3, 1, 1],
                vec![2, 3, 2, 3, 1, 1],
                vec![2, 2, 3, 3, 1, 1],
                vec![3, 3, 2, 1, 2, 1],
                vec![3, 2, 3, 1, 2, 1],
                vec![2, 3, 3, 1, 2, 1],
                vec![3, 3, 1, 2, 2, 1],
                vec![3, 1, 3, 2, 2, 1],
                vec![1, 3, 3, 2, 2, 1],
                vec![3, 2, 1, 3, 2, 1],
                vec![2, 3, 1, 3, 2, 1],
                vec![3, 1, 2, 3, 2, 1],
                vec![1, 3, 2, 3, 2, 1],
                vec![2, 1, 3, 3, 2, 1],
                vec![1, 2, 3, 3, 2, 1],
                vec![3, 2, 2, 1, 3, 1],
                vec![2, 3, 2, 1, 3, 1],
                vec![2, 2, 3, 1, 3, 1],
                vec![3, 2, 1, 2, 3, 1],
                vec![2, 3, 1, 2, 3, 1],
                vec![3, 1, 2, 2, 3, 1],
                vec![1, 3, 2, 2, 3, 1],
                vec![2, 1, 3, 2, 3, 1],
                vec![1, 2, 3, 2, 3, 1],
                vec![2, 2, 1, 3, 3, 1],
                vec![2, 1, 2, 3, 3, 1],
                vec![1, 2, 2, 3, 3, 1],
                vec![3, 3, 2, 1, 1, 2],
                vec![3, 2, 3, 1, 1, 2],
                vec![2, 3, 3, 1, 1, 2],
                vec![3, 3, 1, 2, 1, 2],
                vec![3, 1, 3, 2, 1, 2],
                vec![1, 3, 3, 2, 1, 2],
                vec![3, 2, 1, 3, 1, 2],
                vec![2, 3, 1, 3, 1, 2],
                vec![3, 1, 2, 3, 1, 2],
                vec![1, 3, 2, 3, 1, 2],
                vec![2, 1, 3, 3, 1, 2],
                vec![1, 2, 3, 3, 1, 2],
                vec![3, 3, 1, 1, 2, 2],
                vec![3, 1, 3, 1, 2, 2],
                vec![1, 3, 3, 1, 2, 2],
                vec![3, 1, 1, 3, 2, 2],
                vec![1, 3, 1, 3, 2, 2],
                vec![1, 1, 3, 3, 2, 2],
                vec![3, 2, 1, 1, 3, 2],
                vec![2, 3, 1, 1, 3, 2],
                vec![3, 1, 2, 1, 3, 2],
                vec![1, 3, 2, 1, 3, 2],
                vec![2, 1, 3, 1, 3, 2],
                vec![1, 2, 3, 1, 3, 2],
                vec![3, 1, 1, 2, 3, 2],
                vec![1, 3, 1, 2, 3, 2],
                vec![1, 1, 3, 2, 3, 2],
                vec![2, 1, 1, 3, 3, 2],
                vec![1, 2, 1, 3, 3, 2],
                vec![1, 1, 2, 3, 3, 2],
                vec![3, 2, 2, 1, 1, 3],
                vec![2, 3, 2, 1, 1, 3],
                vec![2, 2, 3, 1, 1, 3],
                vec![3, 2, 1, 2, 1, 3],
                vec![2, 3, 1, 2, 1, 3],
                vec![3, 1, 2, 2, 1, 3],
                vec![1, 3, 2, 2, 1, 3],
                vec![2, 1, 3, 2, 1, 3],
                vec![1, 2, 3, 2, 1, 3],
                vec![2, 2, 1, 3, 1, 3],
                vec![2, 1, 2, 3, 1, 3],
                vec![1, 2, 2, 3, 1, 3],
                vec![3, 2, 1, 1, 2, 3],
                vec![2, 3, 1, 1, 2, 3],
                vec![3, 1, 2, 1, 2, 3],
                vec![1, 3, 2, 1, 2, 3],
                vec![2, 1, 3, 1, 2, 3],
                vec![1, 2, 3, 1, 2, 3],
                vec![3, 1, 1, 2, 2, 3],
                vec![1, 3, 1, 2, 2, 3],
                vec![1, 1, 3, 2, 2, 3],
                vec![2, 1, 1, 3, 2, 3],
                vec![1, 2, 1, 3, 2, 3],
                vec![1, 1, 2, 3, 2, 3],
                vec![2, 2, 1, 1, 3, 3],
                vec![2, 1, 2, 1, 3, 3],
                vec![1, 2, 2, 1, 3, 3],
                vec![2, 1, 1, 2, 3, 3],
                vec![1, 2, 1, 2, 3, 3],
                vec![1, 1, 2, 2, 3, 3]
            ]
        );
    }
}

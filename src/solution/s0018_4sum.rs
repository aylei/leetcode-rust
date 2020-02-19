/**
 * [18] 4Sum
 *
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
 *
 * Note:
 *
 * The solution set must not contain duplicate quadruplets.
 *
 * Example:
 *
 *
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 *
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: change to faster N^3 solution... maybe
// this is a N^2 * logN solution, but slower than N^3 solution
// iterate all combinations and the sum of 2 elements, then use one-round hash
use std::collections::BTreeMap;
use std::collections::HashSet;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut map: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        // collect two-sums in asc order, store the index to avoid single number reusing
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                map.entry(nums[i] + nums[j])
                    .or_insert(Vec::new())
                    .push((i, j));
            }
        }
        // find results
        for (&sum, pairs) in map.iter() {
            // avoid duplicates
            if sum > target / 2 {
                break;
            }
            match map.get(&(target - sum)) {
                None => continue,
                // 2-sum + 2-sum == target, then all the possible combination
                // (without index conflicts) is our answer
                Some(subs) => {
                    for pair in pairs.iter() {
                        for sub in subs.iter() {
                            if sub.0 == pair.0
                                || sub.0 == pair.1
                                || sub.1 == pair.0
                                || sub.1 == pair.1
                            {
                                continue;
                            }
                            let mut vec =
                                vec![nums[pair.0], nums[pair.1], nums[sub.0], nums[sub.1]];
                            vec.sort();
                            set.insert(vec);
                        }
                    }
                }
            }
        }
        set.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: build a macro for arbitrary match
    #[test]
    #[ignore]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-1, 0, 0, 1], vec![-2, 0, 0, 2], vec![-2, -1, 1, 2]]
        );
    }
}

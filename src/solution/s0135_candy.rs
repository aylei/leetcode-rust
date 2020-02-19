/**
 * [135] Candy
 *
 * There are N children standing in a line. Each child is assigned a rating value.
 *
 * You are giving candies to these children subjected to the following requirements:
 *
 *
 * 	Each child must have at least one candy.
 * 	Children with a higher rating get more candies than their neighbors.
 *
 *
 * What is the minimum candies you must give?
 *
 * Example 1:
 *
 *
 * Input: [1,0,2]
 * Output: 5
 * Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
 *
 *
 * Example 2:
 *
 *
 * Input: [1,2,2]
 * Output: 4
 * Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
 *              The third child gets 1 candy because it satisfies the above two conditions.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/candy/
// discuss: https://leetcode.com/problems/candy/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut from = 0;
        let mut n = 1;
        let mut last = 1;
        let mut ascending = false;
        for i in 1..ratings.len() {
            if ratings[i] == ratings[i - 1] {
                n += 1;
                from = i;
                ascending = false;
            } else if ratings[i] >= ratings[i - 1] {
                from = i;
                ascending = true;
                last += 1;
                n += last;
            } else {
                if ascending {
                    last = 1;
                    from = i;
                    ascending = false;
                }
                n += (i - from + 1) as i32;
            }
        }
        n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_135() {
        assert_eq!(Solution::candy(vec![3, 2, 1, 2, 3]), 11);
        assert_eq!(Solution::candy(vec![2, 2, 1, 2, 2]), 7);
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
        assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(Solution::candy(vec![1, 2, 2, 2, 2, 2, 2, 0]), 10);
    }
}

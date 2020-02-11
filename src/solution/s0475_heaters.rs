/**
 * [475] Heaters
 *
 * Winter is coming! Your first job during the contest is to design a standard heater with fixed warm radius to warm all the houses.
 *
 * Now, you are given positions of houses and heaters on a horizontal line, find out minimum radius of heaters so that all houses could be covered by those heaters.
 *
 * So, your input will be the positions of houses and heaters seperately, and your expected output will be the minimum radius standard of heaters.
 *
 * Note:
 *
 * <ol>
 * 	Numbers of houses and heaters you are given are non-negative and will not exceed 25000.
 * 	Positions of houses and heaters you are given are non-negative and will not exceed 10^9.
 * 	As long as a house is in the heaters' warm radius range, it can be warmed.
 * 	All the heaters follow your radius standard and the warm radius will the same.
 * </ol>
 *
 *
 *
 * Example 1:
 *
 *
 * Input: [1,2,3],[2]
 * Output: 1
 * Explanation: The only heater was placed in the position 2, and if we use the radius 1 standard, then all the houses can be warmed.
 *
 *
 *
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,4],[1,4]
 * Output: 1
 * Explanation: The two heater was placed in the position 1 and 4. We need to use radius 1 standard, then all the houses can be warmed.
 *
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        use std::i32::MAX;
        let mut houses = houses;
        houses.sort();
        let mut heaters = heaters;
        heaters.sort();
        let mut ret = 0;
        for house in houses {
            let index = heaters.binary_search(&house);
            if let Err(index) = index {
                let left = if index == 0 {
                    MAX
                } else {
                    house - heaters[index - 1]
                };
                let right = if index == heaters.len() {
                    MAX
                } else {
                    heaters[index] - house
                };
                ret = left.min(right).max(ret);
            }
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_475() {}
}

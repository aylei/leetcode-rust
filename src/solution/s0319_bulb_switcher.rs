/**
 * [319] Bulb Switcher
 *
 * There are n bulbs that are initially off. You first turn on all the bulbs. Then, you turn off every second bulb. On the third round, you toggle every third bulb (turning on if it's off or turning off if it's on). For the i-th round, you toggle every i bulb. For the n-th round, you only toggle the last bulb. Find how many bulbs are on after n rounds.
 *
 * Example:
 *
 *
 * Input: 3
 * Output: 1
 * Explanation:
 * At first, the three bulbs are [off, off, off].
 * After first round, the three bulbs are [on, on, on].
 * After second round, the three bulbs are [on, off, on].
 * After third round, the three bulbs are [on, off, off].
 *
 * So you should return 1, because there is only one bulb is on.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        // number i bulb is toggled when round divides i, e.g.
        // bulb 12 is toggled in round 1, 2, 3, 4, 6, 12
        // bulb 12's divisor comes in pairs, for example (2, 6), (3, 4)
        // except for squres, e.g. 9 => 1, 3, 9, so it will be on
        (n as f64).sqrt() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_319() {}
}

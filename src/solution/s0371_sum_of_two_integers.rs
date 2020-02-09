/**
 * [371] Sum of Two Integers
 *
 * Calculate the sum of two integers a and b, but you are not allowed to use the operator + and -.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: a = <span id="example-input-1-1">1</span>, b = <span id="example-input-1-2">2</span>
 * Output: <span id="example-output-1">3</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: a = -<span id="example-input-2-1">2</span>, b = <span id="example-input-2-2">3</span>
 * Output: 1
 *
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // use `sum` to store direct sum without carriers
        // use `carrier` to store carriers
        let mut sum = a;
        let mut carrier = b;
        while sum != 0 && carrier != 0 {
            let tmp_sum = sum;
            sum = sum ^ carrier;
            carrier = tmp_sum & carrier;
            carrier <<= 1;
        }
        if sum == 0 {
            carrier
        } else {
            sum
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_371() {}
}

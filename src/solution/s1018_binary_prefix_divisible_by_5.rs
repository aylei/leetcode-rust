/**
 * [1071] Binary Prefix Divisible By 5
 *
 * Given an array A of 0s and 1s, consider N_i: the i-th subarray from A[0] to A[i] interpreted as a binary number (from most-significant-bit to least-significant-bit.)
 *
 * Return a list of booleans answer, where answer[i] is true if and only if N_i is divisible by 5.
 *
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[0,1,1]</span>
 * Output: <span id="example-output-1">[true,false,false]</span>
 * Explanation:
 * The input numbers in binary are 0, 01, 011; which are 0, 1, and 3 in base-10.  Only the first number is divisible by 5, so answer[0] is true.
 *
 *
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,1,1]</span>
 * Output: <span id="example-output-2">[false,false,false]</span>
 *
 *
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">[0,1,1,1,1,1]</span>
 * Output: <span id="example-output-3">[true,false,false,false,true,false]</span>
 *
 *
 * Example 4:
 *
 *
 * Input: <span id="example-input-4-1">[1,1,1,0,1]</span>
 * Output: <span id="example-output-4">[false,false,false,false,false]</span>
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * 	1 <= A.length <= 30000
 * 	A[i] is 0 or 1
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut ret = vec![];
        let mut n = 0;
        for i in a {
            let remain = (n * 2 + i) % 5;
            ret.push(remain == 0);
            n = remain;
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1071() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            vec![true, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            vec![false, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
            vec![true, false, false, false, true, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1, 0, 1]),
            vec![false, false, false, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![
                1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1
            ]),
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ]
        );
    }
}

/**
 * [415] Add Strings
 *
 * Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.
 *
 * Note:
 * <ol>
 * The length of both num1 and num2 is < 5100.
 * Both num1 and num2 contains only digits 0-9.
 * Both num1 and num2 does not contain any leading zero.
 * You must not use any built-in BigInteger library or convert the inputs to integer directly.
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1: Vec<char> = num1.chars().collect();
        let num2: Vec<char> = num2.chars().collect();
        let mut ret = String::new();
        let mut carrier = 0;
        let mut index1 = (num1.len() - 1) as i32;
        let mut index2 = (num2.len() - 1) as i32;
        while index1 >= 0 && index2 >= 0 {
            let sum = num1[index1 as usize].to_digit(10).unwrap()
                + num2[index2 as usize].to_digit(10).unwrap()
                + carrier;
            let curr = sum % 10;
            carrier = sum / 10;
            ret = curr.to_string() + ret.as_str();
            index1 -= 1;
            index2 -= 1;
        }
        while index1 >= 0 {
            let sum = num1[index1 as usize].to_digit(10).unwrap() + carrier;
            let curr = sum % 10;
            carrier = sum / 10;
            ret = curr.to_string() + ret.as_str();
            index1 -= 1;
        }
        while index2 >= 0 {
            let sum = num2[index2 as usize].to_digit(10).unwrap() + carrier;
            let curr = sum % 10;
            carrier = sum / 10;
            ret = curr.to_string() + ret.as_str();
            index2 -= 1;
        }
        if carrier == 1 {
            ret = 1.to_string() + ret.as_str();
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_415() {
        println!(
            "{}",
            Solution::add_strings("1".to_string(), "9999".to_string())
        );
    }
}

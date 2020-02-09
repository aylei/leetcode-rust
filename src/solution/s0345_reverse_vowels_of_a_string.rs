/**
 * [345] Reverse Vowels of a String
 *
 * Write a function that takes a string as input and reverse only the vowels of a string.
 *
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">"hello"</span>
 * Output: <span id="example-output-1">"holle"</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">"leetcode"</span>
 * Output: <span id="example-output-2">"leotcede"</span>
 * </div>
 *
 * Note:<br />
 * The vowels does not include the letter "y".
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let vowels: Vec<char> = "aeiouAEIOU".chars().collect();
        let mut chars: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, chars.len() - 1);
        while left < right {
            while !vowels.contains(&chars[left]) && left < right {
                left += 1;
            }
            while !vowels.contains(&chars[right]) && left < right {
                right -= 1;
            }
            if left < right {
                let tmp = chars[left];
                chars[left] = chars[right];
                chars[right] = tmp;
                left += 1;
                right -= 1;
            }
        }
        chars.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_345() {}
}

/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 *
 * <img src="http://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" />
 *
 * Example:
 *
 *
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 *
 *
 * Note:
 *
 * Although the above answer is in lexicographical order, your answer could be in any order you want.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // '0' and '1' as placeholder to avoid index shifting
        let table: Vec<(char, Vec<char>)> = vec![
            ('0', vec![]),
            ('1', vec![]),
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ];
        if digits.len() < 1 {
            return vec![];
        }
        let mut combs: Vec<String> = vec![String::with_capacity(digits.len())];
        for ch in digits.chars().into_iter() {
            let chs = &table[ch.to_digit(10).unwrap() as usize].1;
            let mut added: Vec<String> = Vec::with_capacity((chs.len() - 1) * combs.len());
            for comb in combs.iter_mut() {
                for (i, &alphabetic) in chs.iter().enumerate() {
                    if i == chs.len() - 1 {
                        comb.push(alphabetic);
                    } else {
                        let mut new_comb = (*comb).clone();
                        new_comb.push(alphabetic);
                        added.push(new_comb);
                    }
                }
            }
            combs.append(&mut added);
        }
        combs
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["cf", "af", "bf", "cd", "ce", "ad", "ae", "bd", "be"]
        );
    }
}

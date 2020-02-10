use std::collections::HashMap;

/**
 * [383] Ransom Note
 *
 *
 * Given an arbitrary ransom note string and another string containing letters from all the magazines, write a function that will return true if the ransom
 * note can be constructed from the magazines ; otherwise, it will return false.
 *
 *
 * Each letter in the magazine string can only be used once in your ransom note.
 *
 *
 * Note:<br />
 * You may assume that both strings contain only lowercase letters.
 *
 *
 *
 * canConstruct("a", "b") -> false
 * canConstruct("aa", "ab") -> false
 * canConstruct("aa", "aab") -> true
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for c in magazine.chars() {
            if map.contains_key(&c) {
                map.insert(c, map.get(&c).unwrap() + 1);
            } else {
                map.insert(c, 1);
            }
        }
        for c in ransom_note.chars() {
            if !map.contains_key(&c) || map.get(&c).unwrap() < &1 {
                return false;
            }
            map.insert(c, map.get(&c).unwrap() - 1);
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {}
}

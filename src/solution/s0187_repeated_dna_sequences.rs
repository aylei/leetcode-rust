/**
 * [187] Repeated DNA Sequences
 *
 * All DNA is composed of a series of nucleotides abbreviated as A, C, G, and T, for example: "ACGAATTCCG". When studying DNA, it is sometimes useful to identify repeated sequences within the DNA.
 *
 * Write a function to find all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.
 *
 * Example:
 *
 *
 * Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
 *
 * Output: ["AAAAACCCCC", "CCCCCAAAAA"]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-dna-sequences/
// discuss: https://leetcode.com/problems/repeated-dna-sequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
首先想到直接长度为 10 的 sliding window 滑过去加一个 HashSet

但这种方法在空间上和每次操作的耗时上都比较差, 可以转化为四进制或者二进制编码来考虑

A,C,G,T <-> [00, 01, 10, 11]

那就简单很多了, 往后滑动一格不再需要调整整个 substring, 只需要移位, HashSet 也就存个 u32 即可
*/
use std::collections::HashSet;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut seq_code: u32 = 0;
        let mut set: HashSet<u32> = HashSet::new();
        let mut repeat: HashSet<u32> = HashSet::new();
        let mut count = 0;
        for ch in s.chars() {
            seq_code <<= 2;
            match ch {
                'A' => seq_code |= 0_u32,
                'C' => seq_code |= 1_u32,
                'G' => seq_code |= 2_u32,
                'T' => seq_code |= 3_u32,
                _ => unreachable!(),
            }
            // skip first 9 chars
            if count < 9 {
                count += 1;
                continue;
            }
            // mask high 12-bits
            seq_code &= 0b0000_0000_0000_1111_1111_1111_1111_1111;
            if !set.insert(seq_code) {
                repeat.insert(seq_code);
            }
        }
        // bits code to seq string
        repeat
            .iter()
            .map(|&code| {
                let mut substr = String::new();
                let mut code = code;
                for _ in 0..10 {
                    // take the first 2 bits each time
                    substr.push(match code & 0b0000_0000_0000_1100_0000_0000_0000_0000 {
                        0b0000_0000_0000_0000_0000_0000_0000_0000 => 'A',
                        0b0000_0000_0000_0100_0000_0000_0000_0000 => 'C',
                        0b0000_0000_0000_1000_0000_0000_0000_0000 => 'G',
                        0b0000_0000_0000_1100_0000_0000_0000_0000 => 'T',
                        _ => unreachable!(),
                    });
                    code <<= 2;
                }
                substr
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_187() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned()),
            vec_string!["AAAAACCCCC", "CCCCCAAAAA"]
        );
        assert_eq!(
            Solution::find_repeated_dna_sequences("GAGAGAGAGAGA".to_owned()),
            vec_string!["GAGAGAGAGA"]
        );
    }
}

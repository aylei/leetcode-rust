/**
 * [299] Bulls and Cows
 *
 * You are playing the following <a href="https://en.wikipedia.org/wiki/Bulls_and_Cows" target="_blank">Bulls and Cows</a> game with your friend: You write down a number and ask your friend to guess what the number is. Each time your friend makes a guess, you provide a hint that indicates how many digits in said guess match your secret number exactly in both digit and position (called "bulls") and how many digits match the secret number but locate in the wrong position (called "cows"). Your friend will use successive guesses and hints to eventually derive the secret number.
 *
 * Write a function to return a hint according to the secret number and friend's guess, use A to indicate the bulls and B to indicate the cows.
 *
 * Please note that both secret number and friend's guess may contain duplicate digits.
 *
 * Example 1:
 *
 *
 * Input: secret = "1807", guess = "7810"
 *
 * Output: "1A3B"
 *
 * Explanation: 1<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> bull and </span>3<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> cows. The bull is </span>8<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">, the cows are </span>0<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">, </span>1<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> and </span>7<font face="sans-serif, Arial, Verdana, Trebuchet MS">.</font>
 *
 * Example 2:
 *
 *
 * Input: secret = "1123", guess = "0111"
 *
 * Output: "1A1B"
 *
 * Explanation: The 1st 1 <span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">in friend's guess is a bull, the 2nd or 3rd </span>1<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> is a cow</span><span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">.</span>
 *
 * Note: You may assume that the secret number and your friend's guess only contain digits, and their lengths are always equal.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bulls-and-cows/
// discuss: https://leetcode.com/problems/bulls-and-cows/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut s_iter = secret.chars();
        let mut g_iter = guess.chars();
        let mut bucket = vec![0; 10];
        let mut non_match = vec![];
        while let (Some(s), Some(g)) = (s_iter.next(), g_iter.next()) {
            if s == g {
                bulls += 1;
            } else {
                bucket[s.to_digit(10).unwrap() as usize] += 1;
                non_match.push(g.to_digit(10).unwrap() as usize);
            }
        }
        for &idx in non_match.iter() {
            if bucket[idx] > 0 {
                cows += 1;
                bucket[idx] -= 1;
            }
        }
        format!("{}A{}B", bulls, cows)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_299() {
        assert_eq!(
            Solution::get_hint("1807".to_owned(), "7810".to_owned()),
            "1A3B".to_owned()
        );
        assert_eq!(
            Solution::get_hint("1123".to_owned(), "0111".to_owned()),
            "1A1B".to_owned()
        );
    }
}

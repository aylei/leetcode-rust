/**
 * [204] Count Primes
 *
 * Count the number of prime numbers less than a non-negative number, n.
 *
 * Example:
 *
 *
 * Input: 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-primes/
// discuss: https://leetcode.com/problems/count-primes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut is_prime = vec![true; n as usize];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut i = 2;
        while i * i < n {
            if !is_prime[i as usize] {
                i += 1;
                continue;
            }
            let mut j = i * i;
            while j < n {
                is_prime[j as usize] = false;
                j += i;
            }
            i += 1;
        }
        let mut count = 0;
        for &v in is_prime.iter() {
            if v {
                count += 1
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_204() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(5), 2);
        assert_eq!(Solution::count_primes(1), 0);
        assert_eq!(Solution::count_primes(120), 30);
    }
}

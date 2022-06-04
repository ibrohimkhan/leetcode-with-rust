// https://leetcode.com/problems/repeated-dna-sequences/

use crate::Solution;

impl Solution {
    /// Approach 1:
    /// Linear-time Slice Using Substring + HashSet
    /// Time complexity: O((N-L)L)
    /// Space complexity: O((N-L)L)
    ///
    pub fn find_repeated_dna_sequences_1(s: String) -> Vec<String> {
        use std::collections::HashSet;

        const L: usize = 10;
        let n = s.len();

        let mut seen = HashSet::new();
        let mut result = HashSet::new();

        for i in 0..(n - L + 1) {
            let substr = &s[i..(i + L)];

            if seen.contains(substr) {
                result.insert(substr);
            }

            seen.insert(substr);
        }

        result.iter().map(|&item| item.to_string()).collect()
    }

    /// Approach 2:
    /// Rabin-Karp: Constant-time Slice Using Rolling Hash
    /// Time complexity: O(N-L)
    /// Space complexity: O(N-L)
    ///
    pub fn find_repeated_dna_sequences_2(s: String) -> Vec<String> {
        use std::collections::{HashMap, HashSet};

        const L: usize = 10;
        let n = s.len();

        if n <= L {
            return vec![];
        }

        let a = 4;

        #[allow(non_snake_case)]
        let aL = i32::pow(a, L as u32);

        let map = HashMap::from([('A', 0), ('C', 1), ('G', 2), ('T', 3)]);

        let nums: Vec<i32> = s.chars().map(|item| map[&item]).collect();

        let mut h = 0;
        let mut seen = HashSet::new();
        let mut result = HashSet::new();

        for i in 0..(n - L + 1) {
            if i != 0 {
                h = h * a - nums[i - 1] as i32 * aL + nums[i + L - 1] as i32;
            } else {
                for j in 0..L {
                    h = h * a + nums[j] as i32;
                }
            }

            if seen.contains(&h) {
                let substr = &s[i..(i + L)];
                result.insert(substr);
            }

            seen.insert(h);
        }

        result.iter().map(|&item| item.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let result = Solution::find_repeated_dna_sequences_2(s);
        let expected = vec!["CCCCCAAAAA".to_string(), "AAAAACCCCC".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let s = "AAAAAAAAAAAAA".to_string();
        let result = Solution::find_repeated_dna_sequences_2(s);
        let expected = vec!["AAAAAAAAAA".to_string()];
        assert_eq!(result, expected);
    }
}

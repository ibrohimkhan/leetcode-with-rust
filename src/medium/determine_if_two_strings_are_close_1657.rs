// https://leetcode.com/problems/determine-if-two-strings-are-close/

use crate::Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        fn counter(s: String) -> ([i32; 26], std::collections::HashSet<char>) {
            let mut freq = [0; 26];
            
            for ch in s.chars() { freq[ch as usize - 0x61] += 1; }
            freq.sort();

            (freq, s.chars().collect())
        }

        counter(word1) == counter(word2)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let word1 = "abc".to_string();
        let word2 = "bca".to_string();
        let result = Solution::close_strings(word1, word2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let word1 = "a".to_string();
        let word2 = "aa".to_string();
        let result = Solution::close_strings(word1, word2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();
        let result = Solution::close_strings(word1, word2);
        assert_eq!(result, true);
    }
}

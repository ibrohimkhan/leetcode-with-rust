// https://leetcode.com/problems/word-pattern/

use crate::Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.matches(' ').count() + 1 { return false; }

        let mut map = std::collections::HashMap::new();
        let mut set = std::collections::HashSet::new();

        for (word, ch) in s.split_ascii_whitespace().zip(pattern.chars()) {
            if let Some(w) = map.insert(ch, word) {
                if w != word { return false; }
            } else if !set.insert(word) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }
}
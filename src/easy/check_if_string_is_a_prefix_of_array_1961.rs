// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/

use crate::Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut buffer = String::new();
        for word in words {
            buffer.push_str(word.as_str());
            if buffer == s { return true; }
            if s.len() <= buffer.len() { return false; }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "iloveleetcode".to_string();
        let words = vec![
            "i".to_string(),
            "love".to_string(),
            "leetcode".to_string(),
            "apples".to_string(),
        ];

        let result = Solution::is_prefix_string(s, words);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = "iloveleetcode".to_string();
        let words = vec![
            "apples".to_string(),
            "i".to_string(),
            "love".to_string(),
            "leetcode".to_string(),
        ];

        let result = Solution::is_prefix_string(s, words);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s = "a".to_string();
        let words = vec![
            "aa".to_string(),
            "aaaa".to_string(),
            "love".to_string(),
        ];

        let result = Solution::is_prefix_string(s, words);
        assert_eq!(result, false);
    }
}

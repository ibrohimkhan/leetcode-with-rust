// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/

use crate::Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        use std::collections::HashMap;
        
        let length = words.len();
        let mut map = HashMap::new();
        for word in words {
            for ch in word.chars() {
                *map.entry(ch).or_insert(0) += 1;
            }
        }

        map.iter().all(|item| *item.1 % length == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["abc".to_string(), "aabc".to_string(), "bc".to_string()];
        let result = Solution::make_equal(words);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let words = vec!["ab".to_string(), "a".to_string()];
        let result = Solution::make_equal(words);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let words = vec!["b".to_string(), "a".to_string()];
        let result = Solution::make_equal(words);
        assert_eq!(result, false);
    }

    #[test]
    fn test_4() {
        let words = vec![
            "bbbaa".to_string(),
            "cacca".to_string(),
            "ac".to_string(),
            "bc".to_string(),
            "b".to_string(),
        ];
        let result = Solution::make_equal(words);
        assert_eq!(result, true);
    }

    #[test]
    fn test_5() {
        let words = vec!["abbab".to_string()];
        let result = Solution::make_equal(words);
        assert_eq!(result, true);
    }
}

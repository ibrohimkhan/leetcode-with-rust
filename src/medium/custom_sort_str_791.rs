// https://leetcode.com/problems/custom-sort-string/

use crate::Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for key in s.chars() {
            *map.entry(key).or_insert(0) += 1;
        }

        let mut result = String::new();
        for ch in order.chars().into_iter() {
            if map.contains_key(&ch) {
                for _ in 0..map[&ch] {
                    result.push(ch);
                }
                
                map.remove(&ch);
            }
        }

        for ch in s.chars() {
            if map.contains_key(&ch) {
                result.push(ch);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let order = "cba".to_string();
        let s = "abcd".to_string();
        let result = Solution::custom_sort_string(order, s);
        assert_eq!(result, "cbad".to_string());
    }

    #[test]
    fn test_2() {
        let order = "cbafg".to_string();
        let s = "abcd".to_string();
        let result = Solution::custom_sort_string(order, s);
        assert_eq!(result, "cbad".to_string());
    }

    #[test]
    fn test_3() {
        let order = "kqep".to_string();
        let s = "pekeq".to_string();
        let result = Solution::custom_sort_string(order, s);
        assert_eq!(result, "kqeep".to_string());
    }
}
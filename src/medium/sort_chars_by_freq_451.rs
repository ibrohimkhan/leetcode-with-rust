// https://leetcode.com/problems/sort-characters-by-frequency/

use crate::Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        use std::collections::HashMap;

        let mut freq = HashMap::new();
        for ch in s.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }

        let mut v: Vec<_> = freq.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));

        let mut result = String::new();
        for (&key, &val) in v.into_iter() {
            for _ in 0..val {
                result.push(key);
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
        let s = "tree".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "eert".to_string());
    }

    #[test]
    fn test_2() {
        let s = "cccaaa".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "aaaccc".to_string());
    }

    #[test]
    fn test_3() {
        let s = "Aabb".to_string();
        let result = Solution::frequency_sort(s);
        assert_eq!(result, "bbAa".to_string());
    }
}
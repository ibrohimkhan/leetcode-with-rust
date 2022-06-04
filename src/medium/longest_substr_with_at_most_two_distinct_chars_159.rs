// https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        use std::collections::HashMap;

        let s: Vec<char> = s.chars().collect();
        let length = s.len();

        let mut max_length = 0;
        let mut left = 0;
        let mut right = 0;

        let mut map = HashMap::new();
        while right < length {
            let item = s[right];
            map.insert(item, right);

            if map.len() > 2 {
                let value = *map.values().min().unwrap();
                let item = *map.iter().find(|(_, &val)| val == value).unwrap().0;

                map.remove(&item);
                left = value + 1;
            }

            max_length = max_length.max(right - left + 1);
            right += 1;
        }

        max_length as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "eceba".to_string();
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let s = "ccaabbb".to_string();
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_3() {
        let s = "aac".to_string();
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_4() {
        let s = "a".to_string();
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_5() {
        let s = "cdaba".to_string();
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_6() {
        let s = "abaccc".to_string();
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 4);
    }
}

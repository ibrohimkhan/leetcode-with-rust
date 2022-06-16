// https://leetcode.com/problems/check-if-number-has-equal-digit-count-and-digit-value/

use crate::Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for ch in num.chars() {
            *map.entry(ch as u8 - b'0').or_insert(0) += 1;
        }

        for (index, ch) in num.char_indices() {
            let times = match map.get(&(index as u8)) {
                None => 0u8,
                Some(&item) => item,
            };

            let value = ch as u8 - b'0';
            if times != value {
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
        let num = "1210".to_string();
        let result = Solution::digit_count(num);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let num = "030".to_string();
        let result = Solution::digit_count(num);
        assert_eq!(result, false);
    }
}
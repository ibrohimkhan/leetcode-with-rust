// https://leetcode.com/problems/license-key-formatting/

use crate::Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.replace("-", "").to_ascii_uppercase();
        let mut result = String::new();
        
        let mut count = 0;
        for ch in s.chars().rev() {
            if count == k {
                result.insert(0, '-');
                count = 0;
            }

            result.insert(0, ch);
            count += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "5F3Z-2e-9-w".to_string();
        let k = 4;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, String::from("5F3Z-2E9W"));
    }

    #[test]
    fn test_2() {
        let s = "2-5g-3-J".to_string();
        let k = 2;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, String::from("2-5G-3J"));
    }
}
// https://leetcode.com/problems/remove-palindromic-subsequences/

use crate::Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        if s == s.chars().rev().collect::<String>() {
            return 1;
        }

        2
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "ababa".to_string();
        let result = Solution::remove_palindrome_sub(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let s = "abb".to_string();
        let result = Solution::remove_palindrome_sub(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let s = "baabb".to_string();
        let result = Solution::remove_palindrome_sub(s);
        assert_eq!(result, 2);
    }
}
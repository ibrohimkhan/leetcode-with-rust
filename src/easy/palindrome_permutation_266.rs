// https://leetcode.com/problems/palindrome-permutation/

use crate::Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut set = std::collections::HashSet::new();
        
        for ch in s.chars() {
            if !set.insert(ch) {
                set.remove(&ch);
            }
        }

        set.len() <= 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("code");
        let result = Solution::can_permute_palindrome(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_2() {
        let s = String::from("aab");
        let result = Solution::can_permute_palindrome(s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let s = String::from("carerac");
        let result = Solution::can_permute_palindrome(s);
        assert_eq!(result, true);
    }
}
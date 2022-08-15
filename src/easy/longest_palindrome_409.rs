// https://leetcode.com/problems/longest-palindrome/

use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut counter = 0;

        for ch in s.chars() {
            if !set.insert(ch) {
                set.remove(&ch);
                counter += 1;
            }
        }

        counter *= 2;

        if counter < s.len() { 
            counter += 1;
            counter as _ 
        } else { 
            counter as _
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("abccccdd");
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_2() {
        let s = String::from("a");
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 1);
    }
}
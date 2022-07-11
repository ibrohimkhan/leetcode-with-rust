// https://leetcode.com/problems/make-the-string-great/

use crate::Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack = String::new();
        
        for ch in s.chars() {
            let prev = stack.chars().last();

            if stack.len() > 0 && prev.unwrap() != ch && prev.unwrap().to_ascii_lowercase() == ch.to_ascii_lowercase() {
                stack.pop();

            } else {
                stack.push(ch);
            }
        }

        stack
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "leEeetcode".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "leetcode".to_string());
    }

    #[test]
    fn test_2() {
        let s = "abBAcC".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_3() {
        let s = "s".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "s".to_string());
    }
}
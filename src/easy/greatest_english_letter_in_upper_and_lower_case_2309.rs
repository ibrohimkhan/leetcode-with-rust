// https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/

use crate::Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        for ch in ('A'..='Z').rev() {
            if s.contains(ch) && s.contains(ch.to_ascii_lowercase()) {
                return ch.to_string();
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "lEeTcOdE".to_string();
        let result = Solution::greatest_letter(s);
        assert_eq!(result, String::from("E"));
    }

    #[test]
    fn test_2() {
        let s = "arRAzFif".to_string();
        let result = Solution::greatest_letter(s);
        assert_eq!(result, String::from("R"));
    }

    #[test]
    fn test_3() {
        let s = "AbCdEfGhIjK".to_string();
        let result = Solution::greatest_letter(s);
        assert_eq!(result, String::from(""));
    }
}

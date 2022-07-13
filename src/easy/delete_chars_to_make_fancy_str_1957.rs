// https://leetcode.com/problems/delete-characters-to-make-fancy-string/

use crate::Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut result = String::new();

        for (i, &ch) in s.iter().enumerate() {
            if i > 0 && i < s.len() - 1 && s[i - 1] == s[i] && s[i] == s[i + 1] { continue; }
            result.push(ch);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "leeetcode".to_string();
        let result = Solution::make_fancy_string(s);
        assert_eq!(result, String::from("leetcode"));
    }

    #[test]
    fn test_2() {
        let s = "aaabaaaa".to_string();
        let result = Solution::make_fancy_string(s);
        assert_eq!(result, String::from("aabaa"));
    }

    #[test]
    fn test_3() {
        let s = "aab".to_string();
        let result = Solution::make_fancy_string(s);
        assert_eq!(result, String::from("aab"));
    }
}
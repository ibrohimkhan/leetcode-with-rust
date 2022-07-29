// https://leetcode.com/problems/backspace-string-compare/

use crate::Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn backspace(text: String) -> Vec<char> {
            let mut stack = Vec::<char>::new();

            for ch in text.chars() {
                if ch == '#' {
                    stack.pop();
                } else {
                    stack.push(ch);
                }
            }

            stack
        }

        backspace(s) == backspace(t)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        let result = Solution::backspace_compare(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = "ab##".to_string();
        let t = "d#c#".to_string();
        let result = Solution::backspace_compare(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let s = "b#c".to_string();
        let t = "d".to_string();
        let result = Solution::backspace_compare(s, t);
        assert_eq!(result, false);
    }
}

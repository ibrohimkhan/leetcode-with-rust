// https://leetcode.com/problems/generate-parentheses/

use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
            let mut result = vec![];
            
            if open == 0 && close == 0 {
                return vec![s];
            }

            if open > 0 {
                let mut other = back_track(s.to_owned() + "(", open - 1, close + 1);
                result.append(&mut other);
            }

            if close > 0 {
                let mut other = back_track(s.to_owned() + ")", open, close - 1);
                result.append(&mut other);
            }

            result
        }

        back_track("".into(), n, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 3;
        let result = Solution::generate_parenthesis(n);
        assert_eq!(
            result,
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        );
    }

    #[test]
    fn test_2() {
        let n = 1;
        let result = Solution::generate_parenthesis(n);
        assert_eq!(result, vec!["()".to_string()]);
    }
}

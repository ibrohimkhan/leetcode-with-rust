// https://leetcode.com/problems/occurrences-after-bigram/

use crate::Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words: Vec<_> = text
            .split_whitespace()
            .collect();

        let mut result: Vec<String> = Vec::new();
        for i in 2..words.len() {
            let w1 = words[i - 2];
            let w2 = words[i - 1];
            let w3 = words[i];

            if w1 == first && w2 == second {
                result.push(w3.to_string());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let text = "alice is a good girl she is a good student".to_string();
        let first = "a".to_string();
        let second = "good".to_string();

        let result = Solution::find_ocurrences(text, first, second);
        assert_eq!(result, vec!["girl".to_string(), "student".to_string()]);
    }

    #[test]
    fn test_2() {
        let text = "we will we will rock you".to_string();
        let first = "we".to_string();
        let second = "will".to_string();

        let result = Solution::find_ocurrences(text, first, second);
        assert_eq!(result, vec!["we".to_string(), "rock".to_string()]);
    }
}

// https://leetcode.com/problems/bold-words-in-string/

use crate::Solution;

impl Solution {
    pub fn bold_words(words: Vec<String>, s: String) -> String {
        let mut bold = vec![false; s.len() + 1];

        for word in words {
            let word_len = word.len();

            for i in 0..=s.len() - word_len {
                if word == &s[i..i + word_len] {
                    for j in i..i + word_len {
                        bold[j] = true;
                    }
                }
            }
        }

        let mut result = String::new();
        if bold[0] {
            result.push_str("<b>");
        }

        for i in 0..bold.len() - 1 {
            result.push(s.chars().nth(i).unwrap());

            if bold[i] && !bold[i + 1] {
                result.push_str("</b>");
            }
            if !bold[i] && bold[i + 1] {
                result.push_str("<b>");
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
        let words = vec!["ab".to_string(), "bc".to_string()];
        let s = "aabcd".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "a<b>abc</b>d".to_string());
    }

    #[test]
    fn test_2() {
        let words = vec!["ab".to_string(), "cb".to_string()];
        let s = "aabcd".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "a<b>ab</b>cd".to_string());
    }

    #[test]
    fn test_3() {
        let words = vec![
            "ccb".to_string(),
            "b".to_string(),
            "d".to_string(),
            "cba".to_string(),
            "dc".to_string(),
        ];
        let s = "eeaadadadc".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "eeaa<b>d</b>a<b>d</b>a<b>dc</b>".to_string());
    }

    #[test]
    fn test_4() {
        let words = vec![
            "e".to_string(),
            "cab".to_string(),
            "de".to_string(),
            "cc".to_string(),
            "db".to_string(),
        ];
        let s = "cbccceeead".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "cb<b>ccceee</b>ad".to_string());
    }
}

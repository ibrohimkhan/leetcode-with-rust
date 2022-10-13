// https://leetcode.com/problems/find-and-replace-pattern/

use crate::Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn indexed(word: &String) -> String {
            let chars = word.chars().collect::<Vec<_>>();

            let mut result = String::new();
            for ch in word.chars() {
                let position = chars.iter().position(|&x| x == ch).unwrap();
                result.push_str(position.to_string().as_str());
            }

            result
        }

        let indexed_pattern = indexed(&pattern);

        let mut result = vec![];
        for word in words.iter() {
            if indexed(word) == indexed_pattern {
                result.push(word.to_owned());
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
        let words = vec![
            "abc".to_string(),
            "deq".to_string(),
            "mee".to_string(),
            "aqq".to_string(),
            "dkd".to_string(),
            "ccc".to_string(),
        ];

        let pattern = "abb".to_string();
        let result = Solution::find_and_replace_pattern(words, pattern);
        assert_eq!(result, vec!["mee".to_string(), "aqq".to_string()]);
    }

    #[test]
    fn test_2() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        let pattern = "a".to_string();
        let result = Solution::find_and_replace_pattern(words, pattern);
        assert_eq!(
            result,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}

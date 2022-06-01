// https://leetcode.com/problems/replace-words/

use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let set: HashSet<String> = HashSet::from_iter(dictionary);
        let mut result = String::new();

        for word in sentence.split_ascii_whitespace() {
            let mut prefix = String::new();
            
            for index in 1..=word.len() {
                prefix = word[0..index].to_string();
                if set.contains(&prefix) {
                    break;
                }
            }

            if result.len() > 0 {
                result.push(' ');
            }

            result.push_str(prefix.as_str());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    use super::*;

    #[test]
    fn test_1() {
        let dictionary = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
        let sentence = "the cattle was rattled by the battery".to_string();
        let result = Solution::replace_words(dictionary, sentence);
        assert_eq!(result, "the cat was rat by the bat".to_string());
    }

    #[test]
    fn test_2() {
        let dictionary = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let sentence = "aadsfasf absbs bbab cadsfafs".to_string();
        let result = Solution::replace_words(dictionary, sentence);
        assert_eq!(result, "a a b c".to_string());
    }
}

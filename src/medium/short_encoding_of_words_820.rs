// https://leetcode.com/problems/short-encoding-of-words/

use crate::Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut set = words
            .iter()
            .map(|item| item.to_owned())
            .collect::<std::collections::HashSet<String>>();

        for word in &words {
            for i in 1..word.len() {
                let same_word = &word[i..];

                if set.contains(same_word) {
                    set.remove(same_word);
                }
            }
        }

        let mut length = 0;
        for item in set {
            length += item.len() + 1;
        }

        length as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words = vec!["time".to_string(), "me".to_string(), "bell".to_string()];
        let result = Solution::minimum_length_encoding(words);
        assert_eq!(result, 10);
    }

    #[test]
    fn test2() {
        let words = vec!["t".to_string()];
        let result = Solution::minimum_length_encoding(words);
        assert_eq!(result, 2);
    }
}

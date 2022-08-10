// https://leetcode.com/problems/number-of-valid-words-in-a-sentence/

use crate::Solution;

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let rule_1 = |word: &str| {
            word.bytes()
                .all(|ch| !ch.is_ascii_digit())
        };
        let rule_2 = |word: &str| {
            if let Some(position) = word.find('-') {
                0 < position
                    && position < word.len() - 1
                    && word.as_bytes()[position + 1].is_ascii_lowercase()
                    && word[position + 1..].find('-').is_none()
            } else {
                true
            }
        };

        let rule_3 = |word: &str| {
            word[..word.len() - 1]
                .bytes()
                .all(|ch| ch != b'!' && ch != b',' && ch != b'.')
        };

        sentence
            .split_ascii_whitespace()
            .filter(|word| !word.is_empty() && rule_1(word) && rule_2(word) && rule_3(word))
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let sentence = "cat and  dog".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let sentence = "!this  1-s b8d!".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let sentence = "alice and  bob are Playing stone-game10".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 4);
    }
}

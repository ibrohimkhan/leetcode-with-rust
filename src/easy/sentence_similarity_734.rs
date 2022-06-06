// https://leetcode.com/problems/sentence-similarity/

use crate::Solution;

impl Solution {
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        use std::collections::HashSet;

        if sentence1.len() != sentence2.len() {
            return false;
        }

        let mut set = HashSet::new();
        for pair in similar_pairs {
            if pair.is_empty() {
                continue;
            }

            let key = pair[0].to_owned() + ":" + pair[1].as_str();
            set.insert(key);
        }

        for index in 0..sentence1.len() {
            let word1 = sentence1[index].to_owned() + ":" + sentence2[index].to_owned().as_str();
            let word2 = sentence2[index].to_owned() + ":" + sentence1[index].to_owned().as_str();
 
            if !set.is_empty() && !set.contains(&word1) && !set.contains(&word2) && sentence1[index] != sentence2[index] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sentence1 = vec![
            "great".to_string(),
            "acting".to_string(),
            "skills".to_string(),
        ];

        let sentence2 = vec![
            "fine".to_string(),
            "drama".to_string(),
            "talent".to_string(),
        ];

        let similar_pairs = vec![
            vec!["great".to_string(), "fine".to_string()],
            vec!["drama".to_string(), "acting".to_string()],
            vec!["skills".to_string(), "talent".to_string()],
        ];

        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let sentence1 = vec!["great".to_string()];

        let sentence2 = vec!["great".to_string()];

        let similar_pairs = vec![vec![]];

        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let sentence1 = vec!["great".to_string()];

        let sentence2 = vec!["doubleplus".to_string(), "good".to_string()];

        let similar_pairs = vec![vec!["great".to_string(), "doubleplus".to_string()]];

        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert_eq!(result, false);
    }

    #[test]
    fn test_4() {
        let sentence1 = vec![
            "an".to_string(),
            "extraordinary".to_string(),
            "meal".to_string(),
        ];

        let sentence2 = vec!["one".to_string(), "good".to_string(), "dinner".to_string()];

        let similar_pairs = vec![
            vec!["great".to_string(), "good".to_string()],
            vec!["extraordinary".to_string(), "good".to_string()],
            vec!["well".to_string(), "good".to_string()],
            vec!["wonderful".to_string(), "good".to_string()],
            vec!["excellent".to_string(), "good".to_string()],
            vec!["fine".to_string(), "good".to_string()],
            vec!["nice".to_string(), "good".to_string()],
            vec!["any".to_string(), "one".to_string()],
            vec!["some".to_string(), "one".to_string()],
            vec!["unique".to_string(), "one".to_string()],
            vec!["the".to_string(), "one".to_string()],
            vec!["an".to_string(), "one".to_string()],
            vec!["single".to_string(), "one".to_string()],
            vec!["a".to_string(), "one".to_string()],
            vec!["truck".to_string(), "car".to_string()],
            vec!["wagon".to_string(), "car".to_string()],
            vec!["automobile".to_string(), "car".to_string()],
            vec!["auto".to_string(), "car".to_string()],
            vec!["vehicle".to_string(), "car".to_string()],
            vec!["entertain".to_string(), "have".to_string()],
            vec!["drink".to_string(), "have".to_string()],
            vec!["eat".to_string(), "have".to_string()],
            vec!["take".to_string(), "have".to_string()],
            vec!["fruits".to_string(), "meal".to_string()],
            vec!["brunch".to_string(), "meal".to_string()],
            vec!["breakfast".to_string(), "meal".to_string()],
            vec!["food".to_string(), "meal".to_string()],
            vec!["dinner".to_string(), "meal".to_string()],
            vec!["super".to_string(), "meal".to_string()],
            vec!["lunch".to_string(), "meal".to_string()],
            vec!["possess".to_string(), "own".to_string()],
            vec!["keep".to_string(), "own".to_string()],
            vec!["have".to_string(), "own".to_string()],
            vec!["extremely".to_string(), "very".to_string()],
            vec!["actually".to_string(), "very".to_string()],
            vec!["really".to_string(), "very".to_string()],
            vec!["super".to_string(), "very".to_string()],
        ];

        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert_eq!(result, true);
    }
}

// https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/

use crate::Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = vec![words[0].clone()];
        for i in 1..words.len() {
            let mut curr = words[i].chars().collect::<Vec<_>>();
            curr.sort();

            let mut prev = words[i - 1].chars().collect::<Vec<_>>();
            prev.sort();

            if curr != prev {
                result.push(words[i].clone());
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
            "abba".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
            "cd".to_string(),
            "cd".to_string(),
        ];
        let result = Solution::remove_anagrams(words);
        assert_eq!(result, vec!["abba".to_string(), "cd".to_string()])
    }

    #[test]
    fn test_2() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];

        let result = Solution::remove_anagrams(words);
        assert_eq!(
            result,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ]
        )
    }
}

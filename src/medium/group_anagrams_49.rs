// https://leetcode.com/problems/group-anagrams/

use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::<Vec<char>, Vec<String>>::new();

        for item in strs {
            let mut key = item.chars().collect::<Vec<_>>();
            key.sort();

            map.entry(key).or_insert(vec![]).push(item);
        }

        map.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let mut result = Solution::group_anagrams(strs);
        for item in result.iter_mut() {
            item.sort();
        }

        result.sort_by(|a, b| a.len().cmp(&b.len()));

        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let strs = vec!["".to_string()];

        let result = Solution::group_anagrams(strs);
        let expected = vec![vec!["".to_string()]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let strs = vec!["a".to_string()];

        let result = Solution::group_anagrams(strs);
        let expected = vec![vec!["a".to_string()]];

        assert_eq!(result, expected);
    }
}

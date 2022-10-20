// https://leetcode.com/problems/sort-features-by-popularity/

use crate::Solution;

impl Solution {
    pub fn sort_features(mut features: Vec<String>, responses: Vec<String>) -> Vec<String> {
        let mut freq_map = std::collections::HashMap::new();
        for feature in &features {
            freq_map.insert(feature.to_owned(), 0);
        }

        for response in &responses {
            let set = response
                .split_whitespace()
                .map(|item| item.to_string())
                .collect::<std::collections::HashSet<String>>();

            for feature in &features {
                if set.contains(feature) {
                    freq_map.entry(feature.to_owned()).and_modify(|count| *count += 1);
                }
            }
        }

        features.sort_by(|a, b| freq_map.get(b).unwrap().cmp(freq_map.get(a).unwrap()));
        features
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let features = vec![
            "cooler".to_string(),
            "lock".to_string(),
            "touch".to_string(),
        ];

        let responses = vec![
            "i like cooler cooler".to_string(),
            "lock touch cool".to_string(),
            "locker like touch".to_string(),
        ];

        let result = Solution::sort_features(features, responses);
        assert_eq!(
            result,
            vec![
                "touch".to_string(),
                "cooler".to_string(),
                "lock".to_string(),
            ]
        );
    }

    #[test]
    fn test_2() {
        let features = vec![
            "a".to_string(),
            "aa".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];

        let responses = vec![
            "a".to_string(),
            "a aa".to_string(),
            "a a a a a".to_string(),
            "b a".to_string(),
        ];

        let result = Solution::sort_features(features, responses);
        assert_eq!(
            result,
            vec![
                "a".to_string(),
                "aa".to_string(),
                "b".to_string(),
                "c".to_string()
            ]
        );
    }
}

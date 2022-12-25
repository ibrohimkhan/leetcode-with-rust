// https://leetcode.com/problems/top-k-frequent-words/

use crate::Solution;

impl Solution {
    pub fn top_k_frequent_692(words: Vec<String>, k: i32) -> Vec<String> {
        let mut v = words.into_iter().fold(std::collections::HashMap::<String, usize>::new(), |mut freq, item| {
            *freq.entry(item).or_insert(0) += 1;
            freq
        }).into_iter().collect::<Vec<_>>();

        v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        v.into_iter().take(k as usize).map(|a| a.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words: Vec<String> = vec![
            "i".into(),
            "love".into(),
            "leetcode".into(),
            "i".into(),
            "love".into(),
            "coding".into(),
        ];
        let k = 2;
        let result = Solution::top_k_frequent_692(words, k);
        assert_eq!(result, vec!["i".to_string(), "love".to_string()]);
    }
}

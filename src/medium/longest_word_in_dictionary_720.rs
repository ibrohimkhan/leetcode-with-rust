// https://leetcode.com/problems/longest-word-in-dictionary/

use crate::Solution;

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort_unstable_by(|a, b| {
            if a.len() < b.len() { return std::cmp::Ordering::Less; }

            if a.len() == b.len() {
                if a > b { return std::cmp::Ordering::Less; }
                else { return std::cmp::Ordering::Greater; }
            }

            std::cmp::Ordering::Greater
        });

        words.reverse();
        for i in 0..words.len() {
            let item = &words[i];

            for j in 0..item.len() {
                let part: String = item[..=j].into();
                if !words.contains(&part) { break; }

                if j == item.len() - 1 { return item.to_owned(); }
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words: Vec<String> = vec![
            "w".into(),
            "wo".into(),
            "wor".into(),
            "worl".into(),
            "world".into(),
        ];
        let result = Solution::longest_word(words);
        assert_eq!(result, "world".to_string());
    }

    #[test]
    fn test_2() {
        let words: Vec<String> = vec![
            "a".into(),
            "banana".into(),
            "app".into(),
            "appl".into(),
            "ap".into(),
            "apply".into(),
            "apple".into(),
        ];
        let result = Solution::longest_word(words);
        assert_eq!(result, "apple".to_string());
    }
}

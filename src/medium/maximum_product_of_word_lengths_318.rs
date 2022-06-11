// https://leetcode.com/problems/maximum-product-of-word-lengths/

use crate::Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let length = words.len();
        let v = words
            .iter()
            .map(|item| {
                item.as_bytes()
                    .iter()
                    .fold(0u32, |acc, b| acc | 1 << (b - b'a'))
            })
            .collect::<Vec<_>>();

        let mut max_prod = 0;
        for i in 0..length - 1 {
            for j in (i + 1)..length {
                if v[i] & v[j] == 0 {
                    max_prod = max_prod.max(words[i].len() * words[j].len());
                }
            }
        }

        max_prod as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words = vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ];

        let result = Solution::max_product(words);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_2() {
        let words = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ];

        let result = Solution::max_product(words);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let words = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ];

        let result = Solution::max_product(words);
        assert_eq!(result, 0);
    }
}

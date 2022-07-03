// https://leetcode.com/problems/index-pairs-of-a-string/

use crate::Solution;

impl Solution {
    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let len = text.len();

        for i in 0..len {
            for j in i..len + 1 {
                let word = &text[i..j];
                
                if words.contains(&word.to_string()) {
                    result.push(vec![i as i32, (j - 1) as i32]);
                }
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
        let text = "thestoryofleetcodeandme".to_string();
        let words = vec![
            "story".to_string(),
            "fleet".to_string(),
            "leetcode".to_string(),
        ];

        let result = Solution::index_pairs(text, words);
        assert_eq!(result, vec![[3,7],[9,13],[10,17]]);
    }

    #[test]
    fn test_2() {
        let text = "ababa".to_string();
        let words = vec![
            "aba".to_string(),
            "ab".to_string(),
        ];

        let result = Solution::index_pairs(text, words);
        assert_eq!(result, vec![[0,1],[0,2],[2,3],[2,4]]);
    }
}

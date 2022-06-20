// https://leetcode.com/problems/shortest-word-distance/

use crate::Solution;

impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut position1, mut position2) = (-1, -1);
        let mut min_distance = words_dict.len() as i32;

        for index in 0..words_dict.len() {
            if words_dict[index].eq(&word1) {
                position1 = index as i32;
                
            } else if words_dict[index].eq(&word2) {
                position2 = index as i32;
            }
            
            if position1 != -1 && position2 != -1 {
                min_distance = min_distance.min(i32::abs(position1 - position2));
            }
        }

        min_distance
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words_dict = vec![
            "practice".to_string(),
            "makes".to_string(),
            "perfect".to_string(),
            "coding".to_string(),
            "makes".to_string(),
        ];

        let word1 = "coding".to_string();
        let word2 = "practice".to_string();
        let result = Solution::shortest_distance(words_dict, word1, word2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let words_dict = vec![
            "practice".to_string(),
            "makes".to_string(),
            "perfect".to_string(),
            "coding".to_string(),
            "makes".to_string(),
        ];

        let word1 = "makes".to_string();
        let word2 = "coding".to_string();
        let result = Solution::shortest_distance(words_dict, word1, word2);
        assert_eq!(result, 1);
    }
}

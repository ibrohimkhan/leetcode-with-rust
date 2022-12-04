// https://leetcode.com/problems/shortest-word-distance-iii/

use crate::Solution;

impl Solution {
    pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut index, mut shortest) = (-1_i32, words_dict.len() as i32);
        
        for (i, word) in words_dict.iter().enumerate() {
            if *word == word1 || *word == word2 {
                if index != -1 && (word1 == word2 || *word != words_dict[index as usize]) {
                    shortest = shortest.min(i as i32 - index);
                }

                index = i as i32;
            }
        }

        shortest
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let word_dis = vec![
            "practice".to_string(),
            "makes".to_string(),
            "perfect".to_string(),
            "coding".to_string(),
            "makes".to_string(),
        ];

        assert_eq!(
            Solution::shortest_word_distance(word_dis, "makes".to_string(), "coding".to_string()),
            1
        );
    }

    #[test]
    fn test_2() {
        let word_dis = vec![
            "practice".to_string(),
            "makes".to_string(),
            "perfect".to_string(),
            "coding".to_string(),
            "makes".to_string(),
        ];

        assert_eq!(
            Solution::shortest_word_distance(word_dis, "makes".to_string(), "makes".to_string()),
            3
        );
    }

    #[test]
    fn test_3() {
        let word_dis = vec![
            "a".to_string(),
            "b".to_string(),
        ];

        assert_eq!(
            Solution::shortest_word_distance(word_dis, "a".to_string(), "b".to_string()),
            1
        );
    }
}

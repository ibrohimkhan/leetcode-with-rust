// https://leetcode.com/problems/shortest-completing-word/

use std::ascii::AsciiExt;

use crate::Solution;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        use std::collections::HashMap;

        let mut license_map = HashMap::new();
        license_plate
            .chars()
            .filter_map(|ch| {
                if ch.is_alphabetic() {
                    Some(ch.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .for_each(|ch| {
                *license_map.entry(ch).or_insert(0) += 1;
            });

        let mut result = None;
        for word in words {
            let mut word_map = HashMap::new();
            word.chars().for_each(|ch| {
                *word_map.entry(ch).or_insert(0) += 1;
            });

            let mut is_word_completed = true;
            for (key, value) in &license_map {
                let char_amount_in_word = word_map.get(key);
                if char_amount_in_word == None || *char_amount_in_word.unwrap() < *value {
                    is_word_completed = false;
                    break;
                }
            }

            if is_word_completed {
                if result == None {
                    result = Some(word);

                } else {
                    if word.len() < result.as_ref().unwrap().len() {
                        result = Some(word);
                    }
                }
            }
        }

        result.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let license_plate = "1s3 PSt".to_string();
        let words = vec![
            "step".to_string(),
            "steps".to_string(),
            "stripe".to_string(),
            "stepple".to_string(),
        ];

        let result = Solution::shortest_completing_word(license_plate, words);
        assert_eq!(result, String::from("steps"));
    }

    #[test]
    fn test_2() {
        let license_plate = "1s3 456".to_string();
        let words = vec![
            "looks".to_string(),
            "pest".to_string(),
            "stew".to_string(),
            "show".to_string(),
        ];

        let result = Solution::shortest_completing_word(license_plate, words);
        assert_eq!(result, String::from("pest"));
    }
}

// https://leetcode.com/problems/count-vowel-substrings-of-a-string/

use crate::Solution;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        use std::collections::HashSet;

        if word.len() < 5 {
            return 0;
        }
        
        let word: Vec<char> = word.chars().collect();
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        
        let mut set = HashSet::new();
        let mut count = 0;

        for i in 0..word.len() - 4 {
            for j in i..word.len() {
                let ch = word[j];
                if vowels.contains(&ch) {
                    set.insert(ch);

                    if set.len() >= 5 { count += 1; }

                } else {
                    break;
                }
            }

            set.clear();
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let word = "aeiouu".to_string();
        let result = Solution::count_vowel_substrings(word);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let word = "unicornarihan".to_string();
        let result = Solution::count_vowel_substrings(word);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let word = "cuaieuouac".to_string();
        let result = Solution::count_vowel_substrings(word);
        assert_eq!(result, 7);
    }
}
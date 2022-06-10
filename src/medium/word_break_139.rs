// https://leetcode.com/problems/word-break/

use crate::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::{HashSet, HashMap};

        fn brute(s: &String, set: &HashSet<String>, mem: &mut HashMap<String, bool>) -> bool {
            if mem.contains_key(s) {
                return mem[s];
            }
            
            if set.contains(s) {
                mem.insert(s.to_string(), true);
                return true;
            }

            for i in 1..s.len() {
                let left = &s[0..i];
                let right = &s[i..];
                
                if set.contains(right) && brute(&left.to_string(), set, mem) {
                    return true;
                }
            }

            mem.insert(s.to_string(), false);
            false
        }

        brute(&s, &HashSet::from_iter(word_dict), &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        let result = Solution::word_break(s, word_dict);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        let result = Solution::word_break(s, word_dict);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let s = "catsandog".to_string();
        let word_dict = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ];

        let result = Solution::word_break(s, word_dict);
        assert_eq!(result, false);
    }
}

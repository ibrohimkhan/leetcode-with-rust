// https://leetcode.com/problems/verifying-an-alien-dictionary/

use crate::Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = std::collections::HashMap::new();
        for (i, ch) in order.char_indices() {
            map.insert(ch, i);
        }

        for i in 1..words.len() {
            let (prev, curr) = (&words[i - 1], &words[i]);
            
            if prev == curr || curr.starts_with(prev) { continue; }
            if prev.starts_with(curr) { return false; }

            let prev = prev.chars().collect::<Vec<_>>();
            let curr = curr.chars().collect::<Vec<_>>();
            
            let mut j = 0;
            while prev[j] == curr[j] { j += 1; }

            if map[&prev[j]] > map[&curr[j]] { return false; }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        let result = Solution::is_alien_sorted(words, order);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();
        let result = Solution::is_alien_sorted(words, order);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let words = vec!["apple".to_string(), "app".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = Solution::is_alien_sorted(words, order);
        assert_eq!(result, false);
    }
}

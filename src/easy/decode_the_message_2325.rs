// https://leetcode.com/problems/decode-the-message/

use crate::Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut value = b'a';

        for ch in key.chars() {
            if map.contains_key(&ch) || ch == ' ' { continue; }
            map.insert(ch, value as char);
            value += 1;
        }

        let mut result = String::new();
        for ch in message.chars() {
            if map.contains_key(&ch) { result.push(map[&ch]); }
            if ch == ' ' { result.push(' '); }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        let result = Solution::decode_message(key, message);
        assert_eq!(result, String::from("this is a secret"));
    }

    #[test]
    fn test_2() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();
        let result = Solution::decode_message(key, message);
        assert_eq!(result, String::from("the five boxing wizards jump quickly"));
    }
}
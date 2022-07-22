// https://leetcode.com/problems/rearrange-characters-to-make-target-string/

use crate::Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut array_of_s = [0; 26];
        for ch in s.chars() {
            array_of_s[(ch as u8 - b'a') as usize] += 1;
        }

        let mut array_of_target = [0; 26];
        for ch in target.chars() {
            array_of_target[(ch as u8 - b'a') as usize] += 1;
        }

        let mut result = i32::MAX;
        for i in 0..26 {
            if array_of_target[i] == 0 { continue; }
            result = result.min(array_of_s[i] / array_of_target[i]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "ilovecodingonleetcode".to_string();
        let target = "code".to_string();
        let result = Solution::rearrange_characters(s, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let s = "abcba".to_string();
        let target = "abc".to_string();
        let result = Solution::rearrange_characters(s, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let s = "abbaccaddaeea".to_string();
        let target = "aaaaa".to_string();
        let result = Solution::rearrange_characters(s, target);
        assert_eq!(result, 1);
    }
}
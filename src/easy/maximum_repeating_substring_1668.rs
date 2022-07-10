// https://leetcode.com/problems/maximum-repeating-substring/

use crate::Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut buffer = word.clone();
        let mut count = 0;

        let mut index = 1;
        let length = sequence.len() / word.len();

        while index <= length {
            if sequence.contains(&buffer) {
                index += 1;
                count += 1;
                buffer.push_str(word.as_str());
            } else {
                return count;
            }
        }
        
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let sequence = "ababc".to_string();
        let word = "ab".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let sequence = "ababc".to_string();
        let word = "ba".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let sequence = "ababc".to_string();
        let word = "ac".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let sequence = "a".to_string();
        let word = "a".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_5() {
        let sequence = "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string();
        let word = "aaaba".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_6() {
        let sequence = "aaa".to_string();
        let word = "a".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_7() {
        let sequence = "bcabb".to_string();
        let word = "c".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_8() {
        let sequence = "aaaaaa".to_string();
        let word = "aa".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 3);
    }
}

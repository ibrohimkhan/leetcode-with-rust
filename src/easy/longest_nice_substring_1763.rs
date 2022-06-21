// https://leetcode.com/problems/longest-nice-substring/

use crate::Solution;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let mut max_range: (usize, usize) = (0, 0);
        let byte_array = s.as_bytes();

        for i in 0..(s.len() - 1) {
            let (mut lowercase_mask, mut uppercase_mask) = (0u32, 0u32);

            for j in i..s.len() {
                match byte_array[j] >= b'a' {
                    true => lowercase_mask |= 1 << (byte_array[j] - b'a'),
                    false => uppercase_mask |= 1 << (byte_array[j] - b'A'),
                };

                if lowercase_mask == uppercase_mask && (j + 1 - i) > (max_range.1 - max_range.0) {
                    max_range = (i, j + 1);
                }
            }
        }

        String::from_utf8(byte_array[max_range.0..max_range.1].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "YazaAay".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "aAa".to_string());
    }

    #[test]
    fn test_2() {
        let s = "Bb".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "Bb".to_string());
    }

    #[test]
    fn test_3() {
        let s = "c".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "".to_string());
    }
}
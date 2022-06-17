// https://leetcode.com/problems/number-of-lines-to-write-string/

use crate::Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let (mut lines, mut width) = (1, 0);
        
        for ch in s.chars() {
            let length = widths[ch as usize - 97];
            width += length;

            if width > 100 {
                lines += 1;
                width = length;
            }
        }

        vec![lines, width]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let widths = vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];

        let s = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = Solution::number_of_lines(widths, s);
        assert_eq!(result, vec![3, 60]);
    }

    #[test]
    fn test_2() {
        let widths = vec![
            4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];

        let s = "bbbcccdddaaa".to_string();
        let result = Solution::number_of_lines(widths, s);
        assert_eq!(result, vec![2, 4]);
    }
}

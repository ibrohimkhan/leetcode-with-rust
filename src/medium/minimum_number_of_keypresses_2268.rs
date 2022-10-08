// https://leetcode.com/problems/minimum-number-of-keypresses/

use crate::Solution;

impl Solution {
    pub fn minimum_keypresses(s: String) -> i32 {
        let mut freq = [0; 26];
        for ch in s.chars() {
            freq[(ch as u8 - 97) as usize] += 1;
        }

        freq.sort_by(|a, b| b.cmp(a));

        let mut total = 0;
        for i in 0..26 {
            if freq[i] == 0 { break; }
            total += freq[i] * ((i + 9) / 9);
        }

        total as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "apple".to_string();
        let result = Solution::minimum_keypresses(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let s = "abcdefghijkl".to_string();
        let result = Solution::minimum_keypresses(s);
        assert_eq!(result, 15);
    }
}

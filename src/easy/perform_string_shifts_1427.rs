// https://leetcode.com/problems/perform-string-shifts/

use crate::Solution;

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let k = shift
            .iter()
            .map(|item| item[1] as usize * if item[0] == 0 { 1 } else { s.len() - 1 })
            .sum::<usize>()
            % s.len();

        s[k..].to_string() + &s[..k]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "abc".to_string();
        let shift = vec![vec![0, 1], vec![1, 2]];
        let result = Solution::string_shift(s, shift);
        assert_eq!(result, String::from("cab"));
    }

    #[test]
    fn test_2() {
        let s = "abcdefg".to_string();
        let shift = vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]];
        let result = Solution::string_shift(s, shift);
        assert_eq!(result, String::from("efgabcd"));
    }
}

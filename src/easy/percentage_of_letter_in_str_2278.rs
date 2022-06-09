// https://leetcode.com/problems/percentage-of-letter-in-string/

use crate::Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let count = s.chars().filter(|&item| item == letter).count();
        (count as f32 * 100.0 / s.len() as f32) as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "foobar".to_string();
        let letter = 'o';
        let result = Solution::percentage_letter(s, letter);
        assert_eq!(result, 33);
    }

    #[test]
    fn test_2() {
        let s = "jjjj".to_string();
        let letter = 'k';
        let result = Solution::percentage_letter(s, letter);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let s = "vmvvvvvzrvvpvdvvvvyfvdvvvvpkvvbvvkvvfkvvvkvbvvnvvomvzvvvdvvvkvvvvvvvvvlvcvilaqvvhoevvlmvhvkvtgwfvvzy".to_string();
        let letter = 'v';
        let result = Solution::percentage_letter(s, letter);
        assert_eq!(result, 59);
    }

    #[test]
    fn test_4() {
        let s = "sgawtb".to_string();
        let letter = 's';
        let result = Solution::percentage_letter(s, letter);
        assert_eq!(result, 16);
    }
}
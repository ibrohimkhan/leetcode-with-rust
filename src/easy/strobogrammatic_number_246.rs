// https://leetcode.com/problems/strobogrammatic-number/

use crate::Solution;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let num = num.chars().collect::<Vec<_>>();
        let rotatable_digits = std::collections::HashMap::<char, char>::from_iter([
            ('0', '0'),
            ('1', '1'),
            ('8', '8'),
            ('6', '9'),
            ('9', '6'),
        ]);

        let mut rotated = vec![];
        for key in num.iter() {
            if !rotatable_digits.contains_key(key) { return false; }
            rotated.push(rotatable_digits[key]);
        }

        num == rotated.iter().rev().map(|item| *item).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let num = String::from("69");
        let result = Solution::is_strobogrammatic(num);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let num = String::from("88");
        let result = Solution::is_strobogrammatic(num);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let num = String::from("692");
        let result = Solution::is_strobogrammatic(num);
        assert_eq!(result, false);
    }
}
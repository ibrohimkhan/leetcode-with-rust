// https://leetcode.com/problems/integer-to-roman/

use crate::Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        let symbols = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut result = String::new();
        for i in 0..values.len() {
            while values[i] <= num {
                num -= values[i];
                result.push_str(symbols[i]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let num = 3;
        let result = Solution::int_to_roman(num);
        assert_eq!(result, "III".to_string())
    }

    #[test]
    fn test_2() {
        let num = 58;
        let result = Solution::int_to_roman(num);
        assert_eq!(result, "LVIII".to_string())
    }

    #[test]
    fn test_3() {
        let num = 1994;
        let result = Solution::int_to_roman(num);
        assert_eq!(result, "MCMXCIV".to_string())
    }
}

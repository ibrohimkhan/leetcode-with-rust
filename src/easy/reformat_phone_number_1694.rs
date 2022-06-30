// https://leetcode.com/problems/reformat-phone-number/

use crate::Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let number = number
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .collect::<String>();

        let mut result = String::new();

        let len = number.len();
        let mut index = 0;
        
        while index < len {
            let remaining = len - index;
            let total_digit = if remaining == 4 || remaining == 2 { 2 } else { 3 };

            result.push_str(&number[index..(index + total_digit)]);
            if remaining > 3 { result.push('-'); }

            index += total_digit;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let number = "1-23-45 6".to_string();
        let result = Solution::reformat_number(number);
        assert_eq!(result, String::from("123-456"));
    }

    #[test]
    fn test_2() {
        let number = "123 4-567".to_string();
        let result = Solution::reformat_number(number);
        assert_eq!(result, String::from("123-45-67"));
    }

    #[test]
    fn test_3() {
        let number = "123 4-5678".to_string();
        let result = Solution::reformat_number(number);
        assert_eq!(result, String::from("123-456-78"));
    }
}

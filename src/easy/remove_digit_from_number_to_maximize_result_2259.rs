// https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/

use crate::Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let nums = number.chars().collect::<Vec<_>>();
        
        let mut i = 0;
        while i < number.len() - 1 {
            if nums[i] == digit && nums[i] < nums[i + 1] {
                return number[..i].to_string() + number[i + 1..].as_ref();
            }

            i += 1;
        }

        i = number.len() - 1;
        while i >= 0 {
            if nums[i] == digit {
                return number[..i].to_string() + number[i + 1..].as_ref();
            }

            i -= 1;
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let number = "123".to_string();
        let digit = '3';
        let result = Solution::remove_digit(number, digit);
        assert_eq!(result, String::from("12"));
    }

    #[test]
    fn test_2() {
        let number = "1231".to_string();
        let digit = '1';
        let result = Solution::remove_digit(number, digit);
        assert_eq!(result, String::from("231"));
    }

    #[test]
    fn test_3() {
        let number = "551".to_string();
        let digit = '5';
        let result = Solution::remove_digit(number, digit);
        assert_eq!(result, String::from("51"));
    }
}

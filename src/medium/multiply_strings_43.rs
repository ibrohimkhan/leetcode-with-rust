// https://leetcode.com/problems/multiply-strings/

use crate::Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut result = vec![0; num1.len() + num2.len()];

        for (i, ch1) in num1.chars().rev().enumerate() {
            let x = ch1.to_digit(10).unwrap() as i32;
            
            for (j, ch2) in num2.chars().rev().enumerate() {
                let y = ch2.to_digit(10).unwrap() as i32;

                let prod = x * y + result[i + j];
                let current_number = prod % 10;
                let carry = prod / 10;

                result[i + j] = current_number;
                result[i + j + 1] += carry;
            }
        }

        result
            .into_iter()
            .rev()
            .map(|item| item.to_string())
            .collect::<String>()
            .trim_start_matches("0")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_1() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "6".to_string());
    }

    #[test]
    fn test_2() {
        let num1 = "123".to_string();
        let num2 = "456".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "56088".to_string());
    }

    #[test]
    fn test_3() {
        let num1 = "0".to_string();
        let num2 = "456".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "0".to_string());
    }
}

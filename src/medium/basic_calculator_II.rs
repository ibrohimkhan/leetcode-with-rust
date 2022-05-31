// https://leetcode.com/problems/basic-calculator-ii/

use crate::Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.trim().to_string().trim_start_matches("0").to_string();
        if s.is_empty() {
            return 0;
        }

        let item = s.parse::<i32>();
        if item.is_ok() {
            return item.unwrap();
        }

        let mut current_number: i32 = 0;
        let mut operation = '+';

        let mut stack = Vec::new();

        for ch in (s + "!").chars() {
            match ch {
                ' ' => continue,
                ('0'..='9') => current_number = 10 * current_number + ch.to_digit(10).unwrap() as i32,
                _ => {
                    match operation {
                        '-' => stack.push(-current_number),
                        '+' => stack.push(current_number),
                        '*' => {
                            let item = stack.pop().unwrap();
                            stack.push(item * current_number);
                        }
                        '/' => {
                            let item = stack.pop().unwrap();
                            stack.push(item / current_number);
                        }
                        _ => (),
                    };

                    operation = ch;
                    current_number = 0;
                }
            }
        }

        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "3+2*2".to_string();
        let result = Solution::calculate(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_2() {
        let s = " 3/2 ".to_string();
        let result = Solution::calculate(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let s = " 3+5 / 2 ".to_string();
        let result = Solution::calculate(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_4() {
        let s = "42".to_string();
        let result = Solution::calculate(s);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_5() {
        let s = "12-3*4".to_string();
        let result = Solution::calculate(s);
        assert_eq!(result, 0);
    }
}

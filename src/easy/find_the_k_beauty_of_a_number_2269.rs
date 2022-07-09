// https://leetcode.com/problems/find-the-k-beauty-of-a-number/

use crate::Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut value = String::new();
        let mut counter = 0;

        for ch in num.to_string().chars() {
            if value.len() != k as usize {
                value.push(ch);
                continue;
            }

            let digit = value.parse::<i32>().unwrap();
            if digit > 0 && num % digit == 0 {
                counter += 1;
            }

            value.remove(0);
            value.push(ch);
        }

        if !value.is_empty() {
            let digit = value.parse::<i32>().unwrap();
            
            if digit > 0 && num % digit == 0 {
                counter += 1;
            }
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let num = 240;
        let k = 2;
        let result = Solution::divisor_substrings(num, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let num = 430043;
        let k = 2;
        let result = Solution::divisor_substrings(num, k);
        assert_eq!(result, 2);
    }
}

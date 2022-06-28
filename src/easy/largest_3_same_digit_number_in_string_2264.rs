// https://leetcode.com/problems/largest-3-same-digit-number-in-string/

use crate::Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max_good_int = String::new();
        let mut buffer = Vec::with_capacity(3);

        for ch in num.chars() {
            if buffer.is_empty() {
                buffer.push(ch);
                continue;
            }

            if *buffer.last().unwrap() != ch {
                buffer.clear();
            }

            buffer.push(ch);
            
            if buffer.len() == 3 {
                let value = buffer.iter().collect::<String>();
                
                if max_good_int.is_empty() || value.parse().unwrap_or(0) > max_good_int.parse().unwrap() {
                    max_good_int = value;
                }
            }
        }

        max_good_int
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let num = String::from("6777133339");
        let result = Solution::largest_good_integer(num);
        assert_eq!(result, String::from("777"));
    }

    #[test]
    fn test_2() {
        let num = String::from("2300019");
        let result = Solution::largest_good_integer(num);
        assert_eq!(result, String::from("000"));
    }

    #[test]
    fn test_3() {
        let num = String::from("42352338");
        let result = Solution::largest_good_integer(num);
        assert_eq!(result, String::from(""));
    }
}
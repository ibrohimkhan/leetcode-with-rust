// https://leetcode.com/problems/reverse-integer/

use crate::Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result = 0;
        while x != 0 {
            let rem = x % 10;
            x /= 10;

            if result > i32::MAX / 10 || (result == i32::MAX / 10 && rem > 7) {
                return 0;
            }

            if result < i32::MIN / 10 || (result == i32::MIN / 10 && rem < -8) {
                return 0;
            }

            result = result * 10 + rem;
        }

        result
    }

    pub fn reverse_2(x: i32) -> i32 {
        x.signum()
            * match x
                .abs()
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
            {
                Ok(value) => value,
                Err(_) => 0,
            }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let x = 123;
        let result = Solution::reverse_2(x);
        assert_eq!(result, 321);
    }

    #[test]
    fn test_2() {
        let x = -123;
        let result = Solution::reverse(x);
        assert_eq!(result, -321);
    }

    #[test]
    fn test_3() {
        let x = 120;
        let result = Solution::reverse_2(x);
        assert_eq!(result, 21);
    }
}

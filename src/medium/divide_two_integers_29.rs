// https://leetcode.com/problems/divide-two-integers/

use crate::Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 { return i32::MAX; }

        let mut negatives = 2;

        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        let mut quotient = 0;
        while dividend - divisor <= 0 {
            quotient -= 1;
            dividend -= divisor;
        }

        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }

    pub fn divide2(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 { return i32::MAX; }

        let mut negatives = 2;

        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        const HALF_I32_MIN: i32 = -1073741824;

        let mut quotient = 0;
        while divisor >= dividend {
            let mut power_of_two = -1;
            let mut value = divisor;

            while value >= HALF_I32_MIN && value + value >= dividend {
                value += value;
                power_of_two += power_of_two;
            }

            quotient += power_of_two;
            dividend -= value;
        }

        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let dividend = 10;
        let divisor = 3;
        let result = Solution::divide2(dividend, divisor);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let dividend = 7;
        let divisor = -3;
        let result = Solution::divide2(dividend, divisor);
        assert_eq!(result, -2);
    }
}

// https://leetcode.com/problems/happy-number/

use crate::Solution;

impl Solution {
    // this is rust impl of the solution
    pub fn is_happy(mut n: i32) -> bool {
        fn sum_digits(mut digit: i32) -> i32 {
            let mut sum = 0;
            while digit > 0 {
                sum += (digit % 10) * (digit % 10);
                digit /= 10;
            }

            sum
        }

        let mut set = std::collections::HashSet::new();
        while !set.contains(&n) && n != 1 {
            set.insert(n);
            n = sum_digits(n);
        }

        n == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_happy(2), false);
    }
}

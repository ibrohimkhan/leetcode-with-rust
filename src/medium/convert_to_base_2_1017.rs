// https://leetcode.com/problems/convert-to-base-2/

use crate::Solution;

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 { return 0.to_string(); }

        let mut result = String::new();
        while n != 0 {
            let rem = n % -2;

            if n < 0 && rem != 0 {
                result.push_str((2 + rem).to_string().as_str());
                n = 1 + n / -2;

            } else {
                result.push_str(rem.to_string().as_str());
                n = n / -2;
            }
        }
        
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 2;
        let result = Solution::base_neg2(n);
        assert_eq!(result, String::from("110"));
    }

    #[test]
    fn test_2() {
        let n = 3;
        let result = Solution::base_neg2(n);
        assert_eq!(result, String::from("111"));
    }

    #[test]
    fn test_3() {
        let n = 4;
        let result = Solution::base_neg2(n);
        assert_eq!(result, String::from("100"));
    }
}

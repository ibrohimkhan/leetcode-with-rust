// https://leetcode.com/problems/palindrome-number/

use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        let (mut original, mut reversed) = (x, 0);
        
        while original != 0 {
            reversed = reversed * 10 + original % 10;
            original /= 10;
        }
        
        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let x = 121;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let x = -121;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let x = 12;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, false);
    }
}
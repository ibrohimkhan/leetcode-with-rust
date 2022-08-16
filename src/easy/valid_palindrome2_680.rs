// https://leetcode.com/problems/valid-palindrome-ii/

use crate::Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let (mut left, mut right) = (0, s.len() - 1);

        while left < right {
            if s[left] != s[right] {
                let p1 = s[left..right].iter().collect::<String>();
                let p2 = s[left + 1..right + 1].iter().collect::<String>();

                return p1 == p1.chars().rev().collect::<String>()
                    || p2 == p2.chars().rev().collect::<String>();
            }
            
            left += 1;
            right -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("aba");
        let result = Solution::valid_palindrome(s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = String::from("abca");
        let result = Solution::valid_palindrome(s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let s = String::from("abc");
        let result = Solution::valid_palindrome(s);
        assert_eq!(result, false);
    }
}

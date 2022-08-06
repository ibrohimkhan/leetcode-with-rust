// https://leetcode.com/problems/repeated-substring-pattern/

use crate::Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();

        let mut dp = Vec::with_capacity(len);
        for _ in 0..len {
            dp.push(0);
        }

        for i in 1..len {
            let mut j = dp[i - 1];

            while j > 0 && s[i] != s[j] { 
                j = dp[j - 1]; 
            }

            if s[i] == s[j] { 
                j += 1; 
            }

            dp[i] = j;
        }

        let k = dp[len - 1];
        k != 0 && len % (len - k) == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("abab");
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = String::from("aba");
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s = String::from("abcabcabcabc");
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(result, true);
    }
}
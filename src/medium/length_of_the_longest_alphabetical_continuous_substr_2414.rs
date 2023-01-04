// https://leetcode.com/problems/length-of-the-longest-alphabetical-continuous-substring/

use crate::Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let (mut longest, mut curr_len) = (0, 1);
        
        for w in s.windows(2) {
            let a = w[0] as u8;
            let b = w[1] as u8;

            if a + 1 == b { curr_len += 1; }
            else {
                longest = curr_len.max(longest);
                curr_len = 1;
            }
        }

        longest.max(curr_len)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "abacaba";
        let result = Solution::longest_continuous_substring(s.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let s = "abcde";
        let result = Solution::longest_continuous_substring(s.into());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_3() {
        let s = "pcfftiovoygwwncfgews";
        let result = Solution::longest_continuous_substring(s.into());
        assert_eq!(result, 2);
    }
}

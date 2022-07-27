// https://leetcode.com/problems/check-if-string-is-decomposable-into-value-equal-substrings/

use crate::Solution;

impl Solution {
    pub fn is_decomposable(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let mut has_two = false;
        
        let (mut i, mut j) = (0, 0);
        while j < s.len() {
            while j + 1 < s.len() && s[j + 1] == s[i] { j += 1; }
            
            if (j - i + 1) % 3 == 2 {
                if !has_two { has_two = true; }
                else { return false; }

            } else if (j - i + 1) % 3 == 1 { 
                return false; 
            }

            j += 1;
            i = j;
        }

        has_two
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("000111000");
        let result = Solution::is_decomposable(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_2() {
        let s = String::from("00011111222");
        let result = Solution::is_decomposable(s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let s = String::from("011100022233");
        let result = Solution::is_decomposable(s);
        assert_eq!(result, false);
    }
}
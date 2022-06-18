// https://leetcode.com/problems/di-string-match/

use crate::Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let (mut left, mut right, mut result) = (0, n, vec![0; n + 1]);

        for (i, ch) in s.char_indices() {
            match ch {
                'I' => { result[i] = left as i32; left += 1; }
                _ => { result[i] = right as i32; right -= 1; }
            }
        }

        result[n] = left;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "IDID".to_string();
        let result = Solution::di_string_match(s);
        assert_eq!(result, vec![0, 4, 1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let s = "III".to_string();
        let result = Solution::di_string_match(s);
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let s = "DDI".to_string();
        let result = Solution::di_string_match(s);
        assert_eq!(result, vec![3, 2, 0, 1]);
    }
}

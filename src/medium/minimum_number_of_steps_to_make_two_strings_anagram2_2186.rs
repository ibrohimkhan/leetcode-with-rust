// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/

use crate::Solution;

impl Solution {
    pub fn min_steps_2186(s: String, t: String) -> i32 {
        let mut freq = [0; 26];
        for &ch in s.as_bytes() {
            freq[(ch - b'a') as usize] += 1;
        }

        for &ch in t.as_bytes() {
            freq[(ch - b'a') as usize] -= 1;
        }

        freq.iter().map(|&x| i32::abs(x)).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let t = "coats".to_string();
        let result = Solution::min_steps_2186(s, t);
        assert_eq!(result, 7)
    }

    #[test]
    fn test_2() {
        let s = "night".to_string();
        let t = "thing".to_string();
        let result = Solution::min_steps_2186(s, t);
        assert_eq!(result, 0)
    }
}

// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/

use crate::Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.contains("01")
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("1001");
        let result = Solution::check_ones_segment(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_2() {
        let s = String::from("110");
        let result = Solution::check_ones_segment(s);
        assert_eq!(result, true);
    }
}
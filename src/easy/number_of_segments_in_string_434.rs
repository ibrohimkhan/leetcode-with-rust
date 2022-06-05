// https://leetcode.com/problems/number-of-segments-in-a-string/

use crate::Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "Hello, my name is John".to_string();
        let result = Solution::count_segments(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let s = "Hello".to_string();
        let result = Solution::count_segments(s);
        assert_eq!(result, 1);
    }
}
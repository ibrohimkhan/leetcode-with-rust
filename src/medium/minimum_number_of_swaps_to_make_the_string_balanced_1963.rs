// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/

use crate::Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack = vec![];
        for ch in s.chars() {
            if ch == '[' { stack.push(ch); }
            else if !stack.is_empty() && *stack.last().unwrap() == '[' { stack.pop(); }
            else { stack.push(ch); }
        }

        (stack.len() as i32 / 2 + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "][][";
        let result = Solution::min_swaps(s.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let s = "]]][[[";
        let result = Solution::min_swaps(s.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let s = "[]";
        let result = Solution::min_swaps(s.into());
        assert_eq!(result, 0);
    }
}

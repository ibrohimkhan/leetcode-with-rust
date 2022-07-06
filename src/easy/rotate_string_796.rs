// https://leetcode.com/problems/rotate-string/

use crate::Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && s.repeat(2).contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        let result = Solution::rotate_string(s, goal);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();
        let result = Solution::rotate_string(s, goal);
        assert_eq!(result, false);
    }
}

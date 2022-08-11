// https://leetcode.com/problems/buddy-strings/

use crate::Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        if s == goal {
            let mut seen = std::collections::HashSet::new();
            for ch in s.chars() {
                if seen.contains(&ch) {
                    return true;
                }
                seen.insert(ch);
            }

            return false;
        }

        let (mut a, mut b) = (-1, -1);
        for i in 0..s.len() {
            if s.chars().nth(i) != goal.chars().nth(i) {
                if a == -1 {
                    a = i as i32;
                } else if b == -1 {
                    b = i as i32;
                } else {
                    return false;
                }
            }
        }

        b != -1
            && s.chars().nth(a as usize) == goal.chars().nth(b as usize)
            && s.chars().nth(b as usize) == goal.chars().nth(a as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "ab".to_string();
        let goal = "ba".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = "ab".to_string();
        let goal = "ab".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s = "aa".to_string();
        let goal = "aa".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, true);
    }
}

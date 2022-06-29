// https://leetcode.com/problems/count-asterisks/

use crate::Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut is_closed = false;
        let mut count = 0;

        for ch in s.chars() {
            if ch == '|' { is_closed = !is_closed; }
            if ch == '*' && !is_closed { count += 1; }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "l|*e*et|c**o|*de|".to_string();
        let result = Solution::count_asterisks(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let s = "iamprogrammer".to_string();
        let result = Solution::count_asterisks(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let s = "yo|uar|e**|b|e***au|tifu|l".to_string();
        let result = Solution::count_asterisks(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_4() {
        let s = "*||*|||||*|*|***||*||***|".to_string();
        let result = Solution::count_asterisks(s);
        assert_eq!(result, 3);
    }
}
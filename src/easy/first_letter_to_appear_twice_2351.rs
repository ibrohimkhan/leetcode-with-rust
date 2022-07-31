// https://leetcode.com/problems/first-letter-to-appear-twice/

use crate::Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut visited = std::collections::HashSet::new();
        for ch in s.chars() {
            if visited.contains(&ch) { return ch; }
            visited.insert(ch);
        }

        panic!("s must contain at least one letter that appears twice")
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("abccbaacz");
        let result = Solution::repeated_character(s);
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_2() {
        let s = String::from("abcdd");
        let result = Solution::repeated_character(s);
        assert_eq!(result, 'd');
    }
}
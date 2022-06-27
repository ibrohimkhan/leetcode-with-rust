// https://leetcode.com/problems/thousand-separator/

use crate::Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut n = n.to_string();
        for i in (1..=n.len()).rev().step_by(3).skip(1) {
            n.insert(i, '.');
        }

        n
    }

    pub fn thousand_separator2(n: i32) -> String {
        n.to_string()
            .chars()
            .collect::<Vec<char>>()
            .rchunks(3)
            .rev()
            .map(|item| item.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 987;
        let result = Solution::thousand_separator(n);
        assert_eq!(result, String::from("987"));
    }

    #[test]
    fn test_2() {
        let n = 1234;
        let result = Solution::thousand_separator(n);
        assert_eq!(result, String::from("1.234"));
    }
}

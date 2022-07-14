// https://leetcode.com/problems/reformat-the-string/

use crate::Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        fn reoder(item1: &mut Vec<char>, item2: &mut Vec<char>) -> String {
            let mut result = Vec::new();
            while !item1.is_empty() || !item2.is_empty() {
                if !item1.is_empty() {
                    result.push(item1.remove(0));
                }

                if !item2.is_empty() {
                    result.push(item2.remove(0));
                }
            }

            result.iter().map(|ch| ch.to_string()).collect()
        }

        let mut digits = s
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .collect::<Vec<_>>();

        let mut letters = s
            .chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .collect::<Vec<_>>();

        let diff = letters.len() as i32 - digits.len() as i32;
        if ![-1, 0, 1].contains(&diff) {
            return String::new();
        }

        if digits.len() >= letters.len() {
            reoder(&mut digits, &mut letters)
        } else {
            reoder(&mut letters, &mut digits)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("a0b1c23");
        let result = Solution::reformat(s);
        assert_eq!(result, String::from("0a1b2c3"));
    }

    #[test]
    fn test_2() {
        let s = String::from("leetcode");
        let result = Solution::reformat(s);
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn test_3() {
        let s = String::from("123456789");
        let result = Solution::reformat(s);
        assert_eq!(result, String::from(""));
    }
}

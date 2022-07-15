// https://leetcode.com/problems/minimum-moves-to-convert-string/

use crate::Solution;

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let (length, mut index, mut counter) = (s.len(), 0, 0);
        while index < length {
            if s.chars().nth(index).unwrap() == 'X' {
                counter += 1;
                index += 3;
            } else {
                index += 1;
            }
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("X");
        let result = Solution::minimum_moves(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let s = String::from("XXOX");
        let result = Solution::minimum_moves(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let s = String::from("OOOO");
        let result = Solution::minimum_moves(s);
        assert_eq!(result, 0);
    }
}
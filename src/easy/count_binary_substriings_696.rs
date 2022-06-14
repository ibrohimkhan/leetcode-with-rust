// https://leetcode.com/problems/count-binary-substrings/

use crate::Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut groups = vec![0; s.len()];
        groups[0] = 1;

        let mut index = 0;
        for i in 1..s.len() {
            if s[i - 1] != s[i] {
                index += 1;
                groups[index] = 1;
            } else {
                groups[index] += 1;
            }
        }

        let mut result = 0;
        for i in 1..=index {
            result += groups[i - 1].min(groups[i]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "00110011".to_string();
        let result = Solution::count_binary_substrings(s);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let s = "10101".to_string();
        let result = Solution::count_binary_substrings(s);
        assert_eq!(result, 4);
    }
}
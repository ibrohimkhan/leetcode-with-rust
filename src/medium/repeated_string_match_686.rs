// https://leetcode.com/problems/repeated-string-match/

use crate::Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let (mut count, total_len) = (0, a.len() + b.len());
        let mut temp = String::new();

        while temp.len() < total_len {
            temp.push_str(a.as_str());
            count += 1;
            if temp.contains(&b) { return count; }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let a = "abcd".to_string();
        let b = "cdabcdab".to_string();
        let result = Solution::repeated_string_match(a, b);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let a = "a".to_string();
        let b = "aa".to_string();
        let result = Solution::repeated_string_match(a, b);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let a = "abc".to_string();
        let b = "cabcabca".to_string();
        let result = Solution::repeated_string_match(a, b);
        assert_eq!(result, 4);
    }
}

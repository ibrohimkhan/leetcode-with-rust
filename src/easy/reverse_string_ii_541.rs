// https://leetcode.com/problems/reverse-string-ii/

use crate::Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;

        let mut s = s.chars().collect::<Vec<_>>();
        let length = s.len();

        for index in (0..length).step_by(2 * k) {
            let mut i = index;
            let mut j = (index + k - 1).min(length - 1);
            
            while i < j {
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "abcdefg".to_string();
        let k = 2;
        let result = Solution::reverse_str(s, k);
        assert_eq!(result, String::from("bacdfeg"));
    }

    #[test]
    fn test_2() {
        let s = "abcd".to_string();
        let k = 2;
        let result = Solution::reverse_str(s, k);
        assert_eq!(result, String::from("bacd"));
    }
}
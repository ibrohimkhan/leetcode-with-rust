// https://leetcode.com/problems/optimal-partition-of-string/

use crate::Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut result = 1;
        let mut set = std::collections::HashSet::new();

        for ch in s.chars() {
            if set.contains(&ch) {
                result += 1;
                set.clear();
            }

            set.insert(ch);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "abacaba".to_string();
        let result = Solution::partition_string(s);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let s = "ssssss".to_string();
        let result = Solution::partition_string(s);
        assert_eq!(result, 6);
    }
}
